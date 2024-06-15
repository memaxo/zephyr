use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable(Type),
    Function(Vec<Type>, Option<Type>),
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn insert(&mut self, name: String, symbol_type: SymbolType) {
        let symbol = Symbol { name, symbol_type };
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol)
        } else if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
}

pub struct SymbolTableBuilder {
    current_scope: Box<SymbolTable>,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            current_scope: Box::new(SymbolTable::new(None)),
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Box::new(SymbolTable::new(Some(self.current_scope.clone())));
        self.current_scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent_scope) = self.current_scope.parent.take() {
            self.current_scope = parent_scope;
        }
    }

    pub fn insert_variable(&mut self, name: String, var_type: Type) {
        self.current_scope.insert(name, SymbolType::Variable(var_type));
    }

    pub fn insert_function(&mut self, name: String, param_types: Vec<Type>, return_type: Option<Type>) {
        self.current_scope.insert(name, SymbolType::Function(param_types, return_type));
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.current_scope.lookup(name)
    }

    pub fn build(self) -> SymbolTable {
        *self.current_scope
    }
}