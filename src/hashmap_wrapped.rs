use std::collections::HashMap as OriginalHashMap;
use std::fmt::Debug;
use std::hash::Hash;
use log::info;

pub struct HashMap<K,V> {
    wrapped: OriginalHashMap<K,V>,
    warn_threshold: usize,
}

impl <K,V> HashMap<K,V>
    where
        K: Hash + Eq,
        V: Clone
{
    pub fn new_with_warn_threshold(warn_threshold: usize) -> Self {
        let hashmap = OriginalHashMap::new();
        Self {
            wrapped: hashmap,
            warn_threshold
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.wrapped.len() > self.warn_threshold {
            log::warn!("HashMapWrapped has reached warn threshold of {}", self.warn_threshold);
        }
        self.wrapped.insert(k,v)
    }

}

impl <K,V> std::ops::Deref for HashMap<K,V> {
    type Target = OriginalHashMap<K,V>;
    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl <K,V> std::ops::DerefMut for HashMap<K,V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wrapped
    }
}

impl <K,V> Debug for HashMap<K,V>
    where
        K: Debug,
        V: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.wrapped.fmt(f)
    }
}

impl <K,V> HashMap<K,V> {
    pub fn from_hashmap_with_threshold(wrapped: OriginalHashMap<K,V>, warn_threshold: usize) -> Self {
        Self {
            wrapped,
            warn_threshold
        }
    }
}

