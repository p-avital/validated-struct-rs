use std::{any::Any, collections::HashMap};
pub use validated_struct_macros::*;
#[derive(Default)]
pub struct Validator {
    root: Vec<Box<dyn Fn(&dyn Any) -> bool>>,
    children: HashMap<String, Validator>,
}
impl Validator {
    pub fn validate<K: IntoIterator<Item = S>, S: AsRef<str>, V: Any>(
        &self,
        path: K,
        value: &V,
    ) -> bool {
        let mut path = path.into_iter();
        if let Some(p) = path.next() {
            if let Some(child) = self.children.get(p.as_ref()) {
                child.validate(path, value)
            } else {
                true
            }
        } else {
            self.root.iter().all(|f| f(value))
        }
    }
    pub fn remove_predicates<K: IntoIterator<Item = S>, S: AsRef<str>>(
        &mut self,
        path: K,
    ) -> Vec<Box<dyn Fn(&dyn Any) -> bool>> {
        let mut path = path.into_iter();
        if let Some(p) = path.next() {
            if let Some(child) = self.children.get_mut(p.as_ref()) {
                child.remove_predicates(path)
            } else {
                Vec::new()
            }
        } else {
            let mut value = Vec::new();
            std::mem::swap(&mut self.root, &mut value);
            value
        }
    }
    pub fn add_predicate<K: IntoIterator<Item = S>, S: Into<String>>(
        &mut self,
        path: K,
        value: Box<dyn Fn(&dyn Any) -> bool>,
    ) {
        let mut path = path.into_iter();
        if let Some(p) = path.next() {
            let child = self.children.entry(p.into()).or_default();
            child.add_predicate(path, value);
        } else {
            self.root.push(value)
        }
    }
}
#[derive(Debug)]
pub enum InsertionError {
    SyncInsertNotAvailable,
    #[cfg(feature = "serde_json")]
    JsonErr(serde_json::Error),
    Str(&'static str),
    String(String),
}
impl std::error::Error for InsertionError {}
impl std::fmt::Display for InsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl InsertionError {
    pub fn sync_insert_not_available() -> Self {
        InsertionError::SyncInsertNotAvailable
    }
}
#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for InsertionError {
    fn from(e: serde_json::Error) -> Self {
        InsertionError::JsonErr(e)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum GetError {
    NoMatchingKey,
    TypeMissMatch,
}
impl std::error::Error for GetError {}
impl std::fmt::Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<&'static str> for InsertionError {
    fn from(s: &'static str) -> Self {
        InsertionError::Str(s)
    }
}
impl From<String> for InsertionError {
    fn from(s: String) -> Self {
        InsertionError::String(s)
    }
}
pub trait ValidatedMap {
    fn insert_sync<'d, D: serde::Deserializer<'d>>(
        &mut self,
        _key: &str,
        _value: D,
    ) -> Result<(), InsertionError>
    where
        InsertionError: From<D::Error>,
    {
        Err(InsertionError::sync_insert_not_available())
    }
    fn insert<'d, D: serde::Deserializer<'d>>(
        &mut self,
        key: &str,
        value: D,
    ) -> Result<(), InsertionError>
    where
        InsertionError: From<D::Error>;
    fn get<'a>(&'a self, key: &str) -> Result<&'a dyn Any, GetError>;
}
