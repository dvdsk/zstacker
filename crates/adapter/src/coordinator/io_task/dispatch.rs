// Future work: rewrite using `trie` data structure?
// Future work: track usage per route too. Prevent overloading route

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter;

use tokio::sync::oneshot::Sender;
use tracing::debug;
use zstacker_znp_protocol::commands::Pattern;
use zstacker_znp_protocol::framing::CommandMeta;

use crate::coordinator::ReplyHandler;

#[derive(Debug)]
struct PendingReply {
    awnser_to: Sender<Vec<u8>>,
    reply_meta: CommandMeta,
    reply_pattern: Pattern,
}

#[derive(Debug, Default)]
pub struct Dispatcher {
    status_handlers: HashMap<CommandMeta, PendingReply>,
    normal_handlers: HashMap<CommandMeta, HashMap<Pattern, ReplyHandler>>,
}

#[derive(Debug, thiserror::Error)]
#[error("Could not register request, cmd and pattern already registerd")]
pub struct DuplicateEntry;

impl Dispatcher {
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
        if self.status_handlers.contains_key(&meta) {
            return Err(DuplicateEntry);
        }

        if in_handlers(&self.normal_handlers, &pending) {
            return Err(DuplicateEntry);
        }

        self.status_handlers.insert(
            meta,
            PendingReply {
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
        debug!("registered reply handler");
        res
    }

    pub(crate) fn process_reply(
        &mut self,
        meta: &CommandMeta,
        data: Vec<u8>,
    ) -> Option<()> {
        // Option so we can easily return early with `?`
        if let Some(PendingReply {
            awnser_to,
            reply_meta,
            reply_pattern,
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
}

fn in_handlers(
    handlers: &HashMap<CommandMeta, HashMap<Pattern, ReplyHandler>>,
    pending: &crate::coordinator::PendingSend,
) -> bool {
    handlers
        .get(&pending.reply_meta)
        .map(|patterns| patterns.contains_key(&pending.reply_pattern))
        .unwrap_or(false)
}
