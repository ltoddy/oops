use std::collections::BTreeMap;

use rdev::Key;
use serde::{Deserialize, Serialize};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Statistics(BTreeMap<String, usize>);

impl Statistics {
    pub fn press(&mut self, key: Key) {
        let counter = self.0.entry(format!("{key:?}")).or_insert(0);
        *counter += 1;
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }

    pub fn pretty_format(&self) -> String {
        let mut table = Table::new();
        table.max_column_width = 40;
        table.style = TableStyle::extended();

        self.0.iter().collect::<Vec<_>>().chunks(6).for_each(|chunk| {
            table.add_row(Row::new(
                chunk.iter().map(|(key, value)| TableCell::new(format!("{key}: {value}"))),
            ));
        });

        table.render()
    }

    #[inline]
    pub fn total_presses(&self) -> usize {
        self.0.values().count()
    }
}
