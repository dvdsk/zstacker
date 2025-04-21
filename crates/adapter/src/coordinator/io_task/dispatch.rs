// Future work: rewrite using `trie` data structure?
// Future work: track usage per route too. Prevent overloading route

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter;

use zstacker_znp_protocol::commands::Pattern;
use zstacker_znp_protocol::framing::CommandMeta;

use crate::coordinator::ReplyHandler;

#[derive(Debug, Default)]
pub struct Dispatcher(HashMap<CommandMeta, HashMap<Pattern, ReplyHandler>>);

#[derive(Debug, thiserror::Error)]
#[error("Could not register request, cmd and pattern already registerd")]
pub struct DuplicateEntry;

impl Dispatcher {
    pub(crate) fn register(
        &mut self,
        pending: crate::coordinator::PendingSend,
    ) -> Result<(), DuplicateEntry> {
        dbg!();
        let mut res = Ok(());
        match self.0.entry(pending.reply_meta) {
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

    pub(crate) fn remove(
        &mut self,
        meta: &CommandMeta,
        data: &[u8],
    ) -> Option<ReplyHandler> {
        let patterns = self.0.get_mut(meta)?;
        let matching = {
            let (matching, _) =
                patterns.iter().find(|(pat, _)| pat.matches(data))?;
            matching.clone()
        };
        patterns.remove(&matching)
    }
}
