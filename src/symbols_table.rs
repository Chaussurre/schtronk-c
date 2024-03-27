use crate::ast::parser_builder::Identifier;
use std::collections::HashMap;

#[derive(Default)]
pub struct SymbolsTable {
    pub hashtable: HashMap<Identifier, i32>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable::default()
    }

    pub fn write(&mut self, identifier: Identifier, value: i32) {
        self.hashtable.insert(identifier, value);
    }

    pub fn read(&self, identifier: Identifier) -> Option<i32> {
        self.hashtable.get(&identifier).copied()
    }
}
