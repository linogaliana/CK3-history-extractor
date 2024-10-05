use std::{collections::HashMap, fmt::Debug, rc::Rc};

use super::super::types::{RefOrRaw, Wrapper};

/// A type alias for a game object id.
pub type GameId = u32;

// implementing the Wrapper trait for GameId is overkill, the opaqueness is not needed as it's always going to be a numeric type

/// A type alias for a game string.
/// Roughly meant to represent a raw string from a save file, reference counted so that it exists once in memory.
pub type GameString = Rc<String>;

impl Wrapper<String> for GameString {
    fn wrap(t: String) -> Self {
        Rc::new(t)
    }

    fn get_internal(&self) -> RefOrRaw<String> {
        RefOrRaw::Raw(self.as_ref())
    }
}

/// A value that comes from a save file.
#[derive(Debug, PartialEq, Clone)]
pub enum SaveFileValue {
    String(GameString),
    Object(SaveFileObject),
}

impl SaveFileValue {
    /// Get the value as a string
    ///
    /// # Panics
    ///
    /// Panics if the value is not a string
    ///
    /// # Returns
    ///
    /// A reference to the string
    pub fn as_string(&self) -> GameString {
        match self {
            SaveFileValue::String(s) => s.clone(),
            _ => panic!("Invalid value"),
        }
    }

    /// Get the value as a GameId
    ///
    /// # Panics
    ///
    /// Panics if the value is not a string or the string is not a valid GameId
    ///
    /// # Returns
    ///
    /// The GameId
    pub fn as_id(&self) -> GameId {
        self.as_string().parse::<GameId>().unwrap()
    }

    /// Get the value as a GameObject
    ///
    /// # Panics
    ///
    /// Panics if the value is not an object
    ///
    /// # Returns
    ///
    /// A reference to the object
    pub fn as_object(&self) -> &SaveFileObject {
        match self {
            SaveFileValue::Object(o) => o,
            _ => panic!("Invalid value"),
        }
    }
}

/// An object that comes from a save file.
#[derive(PartialEq, Clone)]
pub enum SaveFileObject {
    Map(GameObject<HashMap<String, SaveFileValue>>),
    Array(GameObject<Vec<SaveFileValue>>),
}

impl SaveFileObject {
    /// Get the name of the object
    pub fn get_name(&self) -> &str {
        match self {
            SaveFileObject::Map(m) => m.get_name(),
            SaveFileObject::Array(a) => a.get_name(),
        }
    }

    /// Get the value as a GameObject map
    pub fn as_map(&self) -> &GameObjectMap {
        match self {
            SaveFileObject::Map(o) => o,
            _ => panic!("Invalid value"),
        }
    }

    pub fn as_map_mut(&mut self) -> &mut GameObjectMap {
        match self {
            SaveFileObject::Map(o) => o,
            _ => panic!("Invalid value"),
        }
    }

    /// Get the value as a GameObject array
    pub fn as_array(&self) -> &GameObjectArray {
        match self {
            SaveFileObject::Array(a) => a,
            _ => panic!("Invalid value"),
        }
    }

    /// Get the value as a mutable GameObject array
    pub fn as_array_mut(&mut self) -> &mut GameObjectArray {
        match self {
            SaveFileObject::Array(a) => a,
            _ => panic!("Invalid value"),
        }
    }

    /// Rename the object
    pub fn rename(&mut self, name: String) {
        match self {
            SaveFileObject::Map(m) => m.rename(name),
            SaveFileObject::Array(a) => a.rename(name),
        }
    }

    /// Check if the object is empty
    pub fn is_empty(&self) -> bool {
        match self {
            SaveFileObject::Map(m) => m.is_empty(),
            SaveFileObject::Array(a) => a.is_empty(),
        }
    }
}

impl Debug for SaveFileObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveFileObject::Map(o) => write!(f, "Map({},{:?})", o.name, o.inner),
            SaveFileObject::Array(o) => write!(f, "Array({},{:?})", o.name, o.inner),
        }
    }
}

/// A game object that stores values as a map.
pub type GameObjectMap = GameObject<HashMap<String, SaveFileValue>>;
/// A game object that stores values as an array.
pub type GameObjectArray = GameObject<Vec<SaveFileValue>>;

/// A trait describing a collection that can be used as storage in [GameObject].
pub trait GameObjectCollection: Debug {
    /// Create a new instance of the collection
    fn new() -> Self;
    /// Check if the collection is empty
    fn is_empty(&self) -> bool;
}

impl GameObjectCollection for HashMap<String, SaveFileValue> {
    fn new() -> Self {
        HashMap::new()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: Debug> GameObjectCollection for Vec<T> {
    fn new() -> Self {
        Vec::new()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Representation of a save file object.
/// These are the main data structure used to store game data.
/// Each belongs to a section, but that is not stored here.
/// Acts like a named dictionary and array, may be either or both or neither.
/// Each has a name, which isn't unique.
/// Holds [SaveFileValue]s, which are either strings or other GameObjects.
#[derive(PartialEq, Clone)]
pub struct GameObject<T: GameObjectCollection> {
    inner: T,
    name: String,
}

impl<T: GameObjectCollection> GameObject<T> {
    /// Create a new GameObject from a name
    pub fn from_name(name: String) -> Self {
        GameObject {
            inner: T::new(),
            name: name,
        }
    }

    /// Create a new empty GameObject
    pub fn new() -> Self {
        GameObject {
            inner: T::new(),
            name: String::new(),
        }
    }

    /// Rename the GameObject
    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    /// Get the name of the GameObject
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Check if the GameObject is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl GameObject<HashMap<String, SaveFileValue>> {
    /// Get the value of a key as a string.
    /// Mainly used for convenience.
    ///
    /// # Panics
    ///
    /// If the key is missing or the value is not a string
    ///
    pub fn get_string_ref(&self, key: &str) -> GameString {
        self.inner.get(key).unwrap().as_string()
    }

    /// Get the value of a key as a GameObject.
    /// Mainly used for convenience.
    ///
    /// # Panics
    ///
    /// If the key is missing or the value is not a GameObject
    ///
    pub fn get_object_ref(&self, key: &str) -> &SaveFileObject {
        self.inner.get(key).unwrap().as_object()
    }

    /// Get the value of a key as a mutable GameObject.
    pub fn get(&self, key: &str) -> Option<&SaveFileValue> {
        self.inner.get(key)
    }

    /// Get the value of a key as a mutable GameObject.
    pub fn insert(&mut self, key: String, value: SaveFileValue) {
        let stored = self.inner.get_mut(&key);
        match stored {
            Some(val) => {
                match val {
                    SaveFileValue::Object(SaveFileObject::Array(arr)) => {
                        arr.push(value);
                    }
                    _ => {
                        let mut arr = GameObjectArray::from_name(key.clone());
                        arr.push(val.clone());
                        arr.push(value);
                        self.inner.insert(key, SaveFileValue::Object(SaveFileObject::Array(arr)));
                    }
                }
            }
            None => {
                self.inner.insert(key, value);
            }
        }
    }
}

impl<'a> IntoIterator for &'a GameObject<HashMap<String, SaveFileValue>> {
    type Item = (&'a String, &'a SaveFileValue);
    type IntoIter = std::collections::hash_map::Iter<'a, String, SaveFileValue>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.inner).into_iter()
    }
}

impl GameObject<Vec<SaveFileValue>> {
    /// Get the value at an index
    pub fn get_index(&self, index: usize) -> Option<&SaveFileValue> {
        self.inner.get(index)
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Push a value to the array
    pub fn push(&mut self, value: SaveFileValue) {
        self.inner.push(value);
    }

    /// Insert a value at an index
    pub fn insert(&mut self, index: usize, value: SaveFileValue) {
        self.inner.insert(index, value);
    }
}

impl<'a> IntoIterator for &'a GameObject<Vec<SaveFileValue>> {
    type Item = &'a SaveFileValue;
    type IntoIter = std::slice::Iter<'a, SaveFileValue>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.inner).into_iter()
    }
}

impl<T: GameObjectCollection> Debug for GameObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = write!(f, "GameObject(name:{}", self.name);
        if r.is_err() {
            return r;
        }
        let r = write!(f, "{:?}", self.inner);
        if r.is_err() {
            return r;
        }
        let r = write!(f, ")");
        return r;
    }
}

// TODO tests
