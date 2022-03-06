use std::collections::BTreeMap;

use rdev::Key;
use serde::{Deserialize, Serialize};

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
}
