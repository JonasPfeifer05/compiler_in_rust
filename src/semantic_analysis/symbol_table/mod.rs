use std::collections::HashMap;
use crate::parser::r#type::ValueType;
use crate::tokenizer::token::Literal;

pub struct SymbolTable {
    scopes: Vec<Scope>
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()]
        }
    }

    pub fn initiate_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn register(&mut self, name: Literal, type_: ValueType) {
        self.scopes.last_mut().unwrap().register(name, type_);
    }

    pub fn get(&self, name: &Literal) -> &ValueType {
        let scope = self.scopes.iter()
            .rev()
            .find(|scope| scope.get(name).is_some()).unwrap();

        scope.get(name).unwrap()
    }
}

pub struct Scope {
    variables: HashMap<Literal, ValueType>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn register(&mut self, name: Literal, type_: ValueType) {
        self.variables.insert(name, type_);
    }

    pub fn get(&self, name: &Literal) -> Option<&ValueType> {
        self.variables.get(name)
    }
}