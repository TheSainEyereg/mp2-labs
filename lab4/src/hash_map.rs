use std::{
    collections::LinkedList,
    hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher},
};

pub struct HashMap<K, V, H = BuildHasherDefault<DefaultHasher>>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
    buckets_count: usize,
    max_load_factor: f64,
    hasher: H,
}

impl<K, V, H> HashMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    pub fn with_capacity_and_hasher(capacity: usize, hasher: H) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(LinkedList::new());
        }

        HashMap {
            buckets,
            size: 0,
            buckets_count: capacity,
            max_load_factor: 2.0,
            hasher,
        }
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.size = 0;
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.load_factor() >= self.max_load_factor {
            self.rehash(self.buckets_count * 2 + 1);
        }

        let index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[index];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }

        bucket.push_back((key, value));
        self.size += 1;
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_bucket_index(key);
        let bucket = &mut self.buckets[index];

        let mut i = 0;
        let mut found = false;

        for (k, _) in bucket.iter() {
            if k == key {
                found = true;
                break;
            }
            i += 1;
        }

        if found {
            let list = std::mem::replace(bucket, LinkedList::new());
            let mut iter = list.into_iter();
            let mut new_list = LinkedList::new();

            for _ in 0..i {
                new_list.push_back(iter.next().unwrap());
            }

            let removed = iter.next().unwrap();

            for item in iter {
                new_list.push_back(item);
            }

            *bucket = new_list;
            self.size -= 1;
            Some(removed.1)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn max_load_factor(&self) -> f64 {
        self.max_load_factor
    }

    pub fn set_max_load_factor(&mut self, factor: f64) {
        if factor > 0.0 {
            self.max_load_factor = factor;
        }
    }

    pub fn load_factor(&self) -> f64 {
        if self.buckets_count == 0 {
            0.0
        } else {
            self.size as f64 / self.buckets_count as f64
        }
    }

    fn get_bucket_index(&self, key: &K) -> usize {
        let mut hasher = self.hasher.build_hasher();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets_count
    }

    fn rehash(&mut self, new_capacity: usize) {
        if new_capacity <= self.buckets_count {
            return;
        }

        let old_buckets = std::mem::replace(&mut self.buckets, Vec::with_capacity(new_capacity));
        self.buckets_count = new_capacity;

        for _ in 0..new_capacity {
            self.buckets.push(LinkedList::new());
        }

        self.size = 0;

        for bucket in old_buckets {
            for (key, value) in bucket {
                self.insert(key, value);
            }
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_bucket_index(key);
        self.buckets[index]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.get_bucket_index(key);
        self.buckets[index]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildHasherDefault::default())
    }
}

impl <K,V> Default for HashMap<K, V> 
where
    K: Hash + Eq

{
    fn default() -> Self {
        HashMap::new(16)
    }
}

impl<K, V> std::ops::Index<&K> for HashMap<K, V>
where
    K: Hash + Eq,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found")
    }
}

impl<K, V> std::ops::IndexMut<&K> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        self.get_mut(key).expect("Key not found")
    }
}
