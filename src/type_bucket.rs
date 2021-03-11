use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt,
};

use std::collections::hash_map;
use std::marker::PhantomData;

/// Prepared key-value pair
pub struct KvPair(TypeId, Box<dyn Storage>, Box<dyn Any>);
// pub enum KvPair {
//     Add(TypeId, Box<dyn Any>),
//     First(TypeId, Box<dyn Any>)
// }

impl KvPair {
    pub fn new<T: 'static>(value: T) -> Self {
        KvPair(
            TypeId::of::<T>(),
            Box::new(Vec::<T>::new()),
            Box::new(value),
        )
    }

    pub fn extract<T: 'static>(self) -> Result<T, Self> {
        let KvPair(key, empty_value, value) = self;
        value
            .downcast()
            .map(|boxed| *boxed)
            .map_err(|e| KvPair(key, empty_value, e))
    }
}

/// A view into an occupied entry in a `TypeBucket`.
#[derive(Debug)]
pub struct OccupiedEntry<'a, T> {
    data: hash_map::OccupiedEntry<'a, TypeId, Box<dyn Any>>,
    marker: PhantomData<fn(T)>,
}

impl<'a, T: 'static> OccupiedEntry<'a, T> {
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &T {
        self.data.get().downcast_ref().unwrap()
    }

    ///Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut().downcast_mut().unwrap()
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    pub fn into_mut(self) -> &'a mut T {
        self.data.into_mut().downcast_mut().unwrap()
    }

    /// Sets the value of the entry, and returns the entry's old value.
    pub fn insert(&mut self, value: T) -> T {
        self.data
            .insert(Box::new(value))
            .downcast()
            .map(|boxed| *boxed)
            .unwrap()
    }

    /// Takes the value out of the entry, and returns it.    
    pub fn remove(self) -> T {
        self.data.remove().downcast().map(|boxed| *boxed).unwrap()
    }
}

/// A view into a vacant entry in a `TypeBucket`.
#[derive(Debug)]
pub struct VacantEntry<'a, T> {
    data: hash_map::VacantEntry<'a, TypeId, Box<dyn Any>>,
    marker: PhantomData<fn(T)>,
}

impl<'a, T: 'static> VacantEntry<'a, T> {
    /// Sets the value of the entry with the key of the `VacantEntry`, and returns a mutable reference to it.
    pub fn insert(self, value: T) -> &'a mut T {
        self.data.insert(Box::new(value)).downcast_mut().unwrap()
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
#[derive(Debug)]
pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

impl<'a, T: 'static> Entry<'a, T> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default()),
        }
    }
}

#[derive(Debug, Default)]
/// The typeBucket container
pub struct TypeBucket {
    // dyn Any is always a Vec<T> where T is TypeId
    // Box<Vec<T>>
    // Box<dyn Any>
    // Box<dyn Storage>
    map: HashMap<TypeId, Box<dyn Storage>>,
}

impl fmt::Debug for dyn Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Storage::debug(self, f)
    }
}

trait Storage {
    fn as_any(&self) -> &dyn Any
    where
        Self: 'static;
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: 'static;
    fn insert_any(&mut self, val: Box<dyn Any>);
    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Storage")
    }
    fn default() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }
}

impl<T: 'static> Storage for Vec<T> {
    #[inline]
    fn as_any(&self) -> &dyn Any
    where
        Self: 'static,
    {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: 'static,
    {
        self
    }
    fn insert_any(&mut self, val: Box<dyn Any>) {
        self.push(*val.downcast().expect("type doesn't match"));
    }
}

trait StorageInfo {
    fn size(&self) -> usize;
}

impl<T: 'static + Sized> StorageInfo for T {
    fn size(&self) -> usize {
        std::mem::size_of::<T>()
    }
}

impl TypeBucket {
    /// Create an empty `TypeBucket`.
    #[inline]
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    /// Insert a prepared `KvPair` into this `TypeBucket`.
    ///
    /// If a value of this type already exists, it will be returned.
    #[track_caller]
    pub fn insert_kv_pair(&mut self, KvPair(key, empty_value, value): KvPair) {
        self.map.entry(key).or_insert(empty_value).insert_any(value)
    }

    /// Insert a value into this `TypeBucket`.
    ///
    /// If a value of this type already exists, it will be returned.
    #[track_caller]
    pub fn insert<T: 'static>(&mut self, val: T) {
        // self.map
        //     .get_or_insert_with(|| HashMap::default())
        //     .entry(TypeId::of::<T>())
        //     .or_insert_with(|| Box::new(Vec::<T>::new()))
        //     .downcast_mut::<Vec<T>>()
        //     .expect("correct type is Vec<T>")
        //     .push(val);
        self.map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Vec::<T>::new()))
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .unwrap()
            .push(val);
    }

    // /// Check if container contains value for type
    // pub fn contains<T: 'static>(&self) -> bool {
    //     self.map
    //         .as_ref()
    //         .and_then(|m| m.get(&TypeId::of::<T>()))
    //         .is_some()
    // }

    /// Get a reference to a value previously inserted on this `TypeBucket`.
    pub fn get<T: 'static>(&self) -> &[T] {
        self.map
            .get(&TypeId::of::<T>())
            .map(|boxed_vec| boxed_vec.as_any().downcast_ref::<Vec<T>>().unwrap())
            .map(|vec| vec.as_slice())
            .unwrap_or_else(|| &[])
    }

    // /// Get a mutable reference to a value previously inserted on this `TypeBucket`.
    // pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
    //     self.map
    //         .as_mut()
    //         .and_then(|m| m.get_mut(&TypeId::of::<T>()))
    //         .and_then(|boxed| boxed.downcast_mut())
    // }

    // /// Remove a value from this `TypeBucket`.
    // ///
    // /// If a value of this type exists, it will be returned.
    // pub fn remove<T: 'static>(&mut self) -> Option<T> {
    //     self.map
    //         .as_mut()
    //         .and_then(|m| m.remove(&TypeId::of::<T>()))
    //         .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    // }

    /// Clear the `TypeBucket` of all inserted values.
    #[inline]
    pub fn clear(&mut self) {
        self.map = Default::default();
    }

    // /// Get an entry in the `TypeBucket` for in-place manipulation.
    // pub fn entry<T: 'static>(&mut self) -> Entry<T> {
    //     match self
    //         .map
    //         .get_or_insert_with(|| HashMap::default())
    //         .entry(TypeId::of::<T>())
    //     {
    //         hash_map::Entry::Occupied(e) => Entry::Occupied(OccupiedEntry {
    //             data: e,
    //             marker: PhantomData,
    //         }),
    //         hash_map::Entry::Vacant(e) => Entry::Vacant(VacantEntry {
    //             data: e,
    //             marker: PhantomData,
    //         }),
    //     }
    // }
}

#[test]
fn test_type_map() {
    #[derive(Debug, PartialEq)]
    struct MyType(i32);

    #[derive(Debug, PartialEq, Default)]
    struct MyType2(String);

    let mut map = TypeBucket::new();

    map.insert(5i32);
    map.insert(MyType(10));

    assert_eq!(map.get::<i32>(), &[5i32]);
    // assert_eq!(map.get_mut(), Some(&mut 5i32));

    // assert_eq!(map.remove::<i32>(), Some(5i32));
    // assert!(map.get::<i32>().is_empty());

    assert_eq!(map.get::<bool>(), &[]);
    assert_eq!(map.get::<MyType>(), &[MyType(10)]);

    map.insert(MyType(20));
    assert_eq!(map.get::<MyType>(), &[MyType(10), MyType(20)]);
    // let entry = map.entry::<MyType2>();

    // let mut v = entry.or_insert_with(MyType2::default);

    // v.0 = "Hello".into();

    // assert_eq!(map.get(), Some(&MyType2("Hello".into())));
}
