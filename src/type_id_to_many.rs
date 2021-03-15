use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct TypeIdToMany<T> {
    map: HashMap<TypeId, Vec<T>>,
}

impl<Value> TypeIdToMany<Value> {
    pub fn insert_distinct<Type: 'static>(&mut self, value_to_add: Value)
    where
        Value: PartialEq,
    {
        let entry = self.map.entry(TypeId::of::<Type>()).or_default();
        if !entry.contains(&value_to_add) {
            entry.push(value_to_add);
        }
    }
    pub fn insert_any_distinct(&mut self, type_id: TypeId, value_to_add: Value)
    where
        Value: PartialEq,
    {
        let entry = self.map.entry(type_id).or_default();
        if !entry.contains(&value_to_add) {
            entry.push(value_to_add);
        }
    }
    pub fn get<Type: 'static>(&self) -> &[Value] {
        self.map
            .get(&TypeId::of::<Type>())
            .map_or(&[], |vec| vec.as_slice())
    }
    // ...
}
