#[cfg(not(fx_hash))] use std::{collections::HashMap, hash::Hash};
#[cfg(not(fx_hash))] type Map<K> = HashMap<K, usize>;
#[cfg(fx_hash)] use fx_hash::FxBuildHasher;
#[cfg(fx_hash)] type Map<K> = HashMap<K, usize, FxBuildHasher>;

#[derive(Debug, Clone)]
pub struct VMap<K, V> {
    keys: Map<K>,
    values: Vec<Vec<V>>
}

impl<K: Sized + Hash + PartialEq + Eq, V> VMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            values: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn next_id(&self) -> usize {
        self.values.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        if self.keys.contains_key(&key) {
            let id = self.keys.get(&key).unwrap();
            self.values[*id].push(value);
            *id
        } else {
            let id = self.next_id();
            self.values.push(vec![value]);
            self.keys.insert(key, id);
            id
        }
    }

    pub fn insert_all(&mut self, keys: Vec<K>, values: Vec<V>) -> usize {
        let id = self.next_id();
        self.values.push(values);
        for k in keys {
            self.keys.insert(k, id);
        }
        id
    }

    pub fn get_index(&self, key: &K) -> Option<&usize> {
        self.keys.get(key)
    }

    pub fn get_index_mut(&mut self, key: &K) -> Option<&mut usize> {
        self.keys.get_mut(key)
    }

    pub fn get_value(&self, key: &K) -> Option<&Vec<V>> {
        let Some(i) = self.get_index(key) else { return None; };
        self.get_with_index(*i)
    }

    pub fn get_value_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        let Some(i) = self.get_index(key) else { return None; };
        self.get_with_index_mut(*i)
    }

    pub fn get_with_index(&self, index: usize) -> Option<&Vec<V>> {
        self.values.get(index)
    }

    pub fn get_with_index_mut(&mut self, index: usize) -> Option<&mut Vec<V>> {
        self.values.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }
}

// impl<K: Hash + PartialEq + Eq, V> Index<K> for VMap<K, V> {
//     type Output = Vec<V>;
//     fn index(&self, index: K) -> &Self::Output {
//         let i = self.keys[&index];
//         &self.values[i]
//     }
// }

// impl<K, V> Index<usize> for VMap<K, V> {
//     type Output = Vec<V>;
//     fn index(&self, index: usize) -> &Self::Output {
//         self.values[&index]
//     }
// }