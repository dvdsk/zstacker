// Future work: rewrite using `trie` data structure?
// Future work: track usage per route too. Prevent overloading route

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter;
use std::time::{Duration, Instant};

use tokio::sync::oneshot::Sender;
use zstacker_znp_protocol::commands::Pattern;
use zstacker_znp_protocol::framing::CommandMeta;

#[derive(Debug)]
struct PendingStatusReply {
    awnser_to: Sender<Vec<u8>>,
    reply_meta: CommandMeta,
    reply_pattern: Pattern,
}

#[derive(Debug)]
pub struct ReplyHandler {
    last_garbage_collect: Instant,
    status_handlers: HashMap<CommandMeta, PendingStatusReply>,
    normal_handlers: HashMap<CommandMeta, HashMap<Pattern, Sender<Vec<u8>>>>,
}

#[derive(Debug, thiserror::Error)]
#[error("Could not register request, cmd and pattern already registerd")]
pub struct DuplicateEntry;

impl ReplyHandler {
    pub(crate) fn new() -> Self {
        Self {
            last_garbage_collect: Instant::now(),
            status_handlers: HashMap::new(),
            normal_handlers: HashMap::new(),
        }
    }

    pub(crate) fn register(
        &mut self,
        mut pending: crate::coordinator::PendingSend,
    ) -> Result<(), DuplicateEntry> {
        if let Some(meta) = pending.status_reply.take() {
            self.register_for_status_reply(meta, pending)
        } else {
            self.register_for_normal_reply(pending)
        }
    }

    pub(crate) fn register_for_status_reply(
        &mut self,
        meta: CommandMeta,
        pending: crate::coordinator::PendingSend,
    ) -> Result<(), DuplicateEntry> {
        if self
            .status_handlers
            .get(&meta)
            .is_some_and(|h| !h.awnser_to.is_closed())
        {
            return Err(DuplicateEntry);
        }

        if in_handlers(&self.normal_handlers, &pending) {
            return Err(DuplicateEntry);
        }

        self.status_handlers.insert(
            meta,
            PendingStatusReply {
                awnser_to: pending.awnser_to,
                reply_meta: pending.reply_meta,
                reply_pattern: pending.reply_pattern,
            },
        );
        Ok(())
    }

    fn register_for_normal_reply(
        &mut self,
        pending: crate::coordinator::PendingSend,
    ) -> Result<(), DuplicateEntry> {
        let mut res = Ok(());
        match self.normal_handlers.entry(pending.reply_meta) {
            Entry::Occupied(occupied) => {
                match occupied.into_mut().entry(pending.reply_pattern) {
                    Entry::Occupied(_) => res = Err(DuplicateEntry),
                    Entry::Vacant(vacent) => {
                        vacent.insert(pending.awnser_to);
                    }
                }
            }
            Entry::Vacant(vacant_entry) => {
                let patterns =
                    iter::once((pending.reply_pattern, pending.awnser_to))
                        .collect();
                vacant_entry.insert(patterns);
            }
        }
        res
    }

    pub(crate) fn process_reply(
        &mut self,
        meta: &CommandMeta,
        data: Vec<u8>,
    ) -> Option<()> {
        // Option so we can easily return early with `?`
        if let Some(PendingStatusReply {
            awnser_to,
            reply_meta,
            reply_pattern,
            ..
        }) = self.status_handlers.remove(meta)
        {
            match self.normal_handlers.entry(reply_meta) {
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().insert(reply_pattern, awnser_to);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(
                        [(reply_pattern, awnser_to)].into_iter().collect(),
                    );
                }
            }
        } else {
            let patterns = self.normal_handlers.get_mut(meta)?;
            let matching = {
                let (matching, _) =
                    patterns.iter().find(|(pat, _)| pat.matches(&data))?;
                matching.clone()
            };
            if let Some(reply_handler) = patterns.remove(&matching) {
                let _dropped_request_future_is_ok = reply_handler.send(data);
            }
        }

        None
    }

    pub(crate) fn collect_garbage(&mut self) {
        if self.last_garbage_collect.elapsed() < Duration::from_secs(30) {
            return;
        }
        self.last_garbage_collect = Instant::now();

        let expired: Vec<CommandMeta> = self
            .status_handlers
            .iter()
            .filter(|(_, PendingStatusReply { awnser_to, .. })| {
                awnser_to.is_closed()
            })
            .map(|(key, _)| key)
            .cloned()
            .collect();
        for key in expired {
            self.status_handlers.remove(&key);
        }

        for (_, handlers) in self.normal_handlers.iter_mut() {
            let expired: Vec<Pattern> = handlers
                .iter()
                .filter(|(_, handler)| handler.is_closed())
                .map(|(key, _)| key)
                .cloned()
                .collect();
            for key in expired {
                handlers.remove(&key);
            }
        }
    }
}

fn in_handlers(
    handlers: &HashMap<CommandMeta, HashMap<Pattern, Sender<Vec<u8>>>>,
    pending: &crate::coordinator::PendingSend,
) -> bool {
    handlers
        .get(&pending.reply_meta)
        .map(|patterns| {
            patterns
                .get(&pending.reply_pattern)
                .is_some_and(|sender| sender.is_closed())
        })
        .unwrap_or(false)
}
