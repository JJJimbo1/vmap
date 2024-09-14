use std::{collections::HashMap, hash::{BuildHasher, Hash, RandomState}, marker::PhantomData, ops::Index};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VMap<K: PartialEq + Eq + Hash, V, S: Default + BuildHasher = RandomState> {
    keys: HashMap<K, usize>,
    values: Vec<Vec<V>>,
    phantom_data: PhantomData<S>,
}

impl<K: Sized + Hash + PartialEq + Eq, V, S: Default + BuildHasher> VMap<K, V, S> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            values: Vec::new(),
            phantom_data: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            phantom_data: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        if self.keys.contains_key(&key) {
            let id = self.keys.get(&key).unwrap();
            self.values[*id].push(value);
            *id
        } else {
            let id = self.len();
            self.values.push(vec![value]);
            self.keys.insert(key, id);
            id
        }
    }

    pub fn insert_all(&mut self, keys: Vec<K>, values: Vec<V>) -> usize {
        let id = self.len();
        self.values.push(values);
        for k in keys {
            self.keys.insert(k, id);
        }
        id
    }

    pub fn values(&self) -> &Vec<Vec<V>> {
        &self.values
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

impl<K: Hash + PartialEq + Eq, V> Index<K> for VMap<K, V> {
    type Output = Vec<V>;
    fn index(&self, index: K) -> &Self::Output {
        let i = self.keys[&index];
        &self.values[i]
    }
}