#![cfg(test)] // only used in tests at the moment

use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct TypeIdToMany<T> {
    map: HashMap<TypeId, Vec<T>>,
}

impl<Value> TypeIdToMany<Value> {
    pub fn insert<Type: 'static>(&mut self, value_to_add: Value) {
        self.map
            .entry(TypeId::of::<Type>())
            .or_default()
            .push(value_to_add);
    }
    pub fn get<Type: 'static>(&self) -> &[Value] {
        self.map
            .get(&TypeId::of::<Type>())
            .map_or(&[], |vec| vec.as_slice())
    }
    // ...
}
