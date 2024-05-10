use crate::ast::parser_builder::Identifier;
use crate::values::Value;
use std::collections::HashMap;
#[derive(Default)]
pub struct SymbolsTable {
    pub hashtable: HashMap<Identifier, Value>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable::default()
    }

    pub fn write(&mut self, identifier: Identifier, value: Value) {
        self.hashtable.insert(identifier, value);
    }

    pub fn read(&self, identifier: Identifier) -> &Value {
        self.hashtable.get(&identifier).unwrap_or(&Value::NOTHING)
    }
}
