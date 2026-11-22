use crate::{Error, Result};
use prybar_io::Reader;
use std::cell::Cell;
use std::ops::Deref;

/// Slack so tiny files still get room for their few names.
const BUDGET_FLOOR: usize = 1 << 16;

/// Reader plus a work budget for one parse. Tables may legally point at
/// shared data, so a hostile file can nest overlapping tables and turn a
/// small input into a huge IR; every record and name is charged here and
/// the parse fails once it costs more than a few times the input size.
pub struct Source<'a> {
    pub reader: Reader<'a>,
    budget: Cell<usize>,
}

impl<'a> Source<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let budget = data.len().saturating_mul(4).saturating_add(BUDGET_FLOOR);
        Self {
            reader: Reader::new(data),
            budget: Cell::new(budget),
        }
    }

    pub fn charge(&self, cost: usize) -> Result<()> {
        let left = self.budget.get().checked_sub(cost).ok_or(Error::Invalid {
            what: "parse budget exceeded by",
            value: cost as i64,
        })?;
        self.budget.set(left);
        Ok(())
    }
}

impl<'a> Deref for Source<'a> {
    type Target = Reader<'a>;

    fn deref(&self) -> &Reader<'a> {
        &self.reader
    }
}
