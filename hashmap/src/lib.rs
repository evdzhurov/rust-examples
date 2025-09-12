// Source:https://viniciusx.com/blog/building-a-hash-map/

use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash, Hasher, RandomState};

pub struct Map<K, V, S = RandomState> {
    build_hasher: S,
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

// Constructor with user-provided build hasher
impl<K, V, S> Map<K, V, S> {
    pub fn with_hasher(build_hasher: S) -> Self {
        Self {
            build_hasher,
            buckets: Vec::new(),
            len: 0,
        }
    }
}

// Constructor with default build hasher
impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self::with_hasher(Default::default())
    }
}

// Generic impl block with trait bounds
impl<K, V, S> Map<K, V, S>
where
    K: Hash + Eq, // Keys must be hashable and comparable (to resolve collisions)
    S: BuildHasher,
{
    const BUCKET_SIZE: usize = 8;

    fn bucket_for<Q>(&self, key: &Q) -> usize
    where
        // K: Borrow<Q> allows us to lookup by value that is a borrowed presentation of the key
        // (e.g. str is a borrowed representation (a view) into a String key).
        // So we don't need to allocate a String for every lookup.
        // Also guarantees that the borrowed representation behaves exactly
        // as the owned type with respect to Eq, Ord, Hash.
        K: Borrow<Q>,
        Q: Hash + ?Sized, // ?Sized = Dynamically Sized Type (DST)
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);

        (hasher.finish() as usize) % self.buckets.len()
    }

    fn grow(&mut self) {
        let old_bucket_count = self.buckets.len();

        // .max(4) computes max(new_bucket_count, 4),
        // which means "double the size, but make it at least 4"
        let new_bucket_count = (self.buckets.len() * 2).max(4);

        self.buckets.resize_with(new_bucket_count, Vec::new);

        // Rehash and move KV-pairs to their new buckets
        for old_bucket in 0..old_bucket_count {
            let mut index = 0;
            while index < self.buckets[old_bucket].len() {
                let new_bucket = self.bucket_for(&self.buckets[old_bucket][index].0);
                if new_bucket != old_bucket {
                    // index does not change, but the element pointed to by index will change
                    // after removal
                    let pair = self.buckets[old_bucket].swap_remove(index);
                    self.buckets[new_bucket].push(pair);
                } else {
                    index += 1;
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() {
            self.grow();

            let bucket = self.bucket_for(&key);
            self.buckets[bucket].push((key, value));
            self.len += 1;

            return None;
        }

        let bucket = self.bucket_for(&key);
        let pair = self.buckets[bucket].iter_mut().find(|(k, _)| *k == key);
        match pair {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                self.buckets[bucket].push((key, value));
                self.len += 1;

                // Grow the hashmap if there are more than BUCKET_SIZE pairs per bucket.
                if Self::BUCKET_SIZE * self.buckets.len() < self.len {
                    self.grow();
                }

                None
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let b = self.bucket_for(key);
        let bucket = &mut self.buckets[b];

        if let Some(i) = bucket.iter().position(|(k, _)| k.borrow() == key) {
            self.len -= 1;
            // Swap remove works in O(1) but does not preserve the order of vector elements (which does not matter for a bucket)
            let (_, v) = bucket.swap_remove(i);
            Some(v)
        } else {
            None
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket = self.bucket_for(key);
        self.buckets[bucket]
            .iter()
            .find_map(|pair| (key == pair.0.borrow()).then_some(&pair.1))
    }
}

impl<K, V, S> Debug for Map<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_map();
        for pair in self.buckets.iter().flat_map(|b| b.iter()) {
            d.entry(&pair.0, &pair.1);
        }
        d.finish()
    }
}

#[test]
fn map_test() {
    let mut map: Map<_, _> = Map::new();
    assert_eq!(map.len, 0);

    map.insert("pi", 31415);
    map.insert("answer", 42);
    map.insert("nine", 4);

    assert_eq!(map.get("pi"), Some(&31415));
    assert_eq!(map.get("answer"), Some(&42));
    assert_eq!(map.get("nine"), Some(&4));

    assert_ne!(map.get("pi"), Some(&1));
    assert_ne!(map.get("answer"), None);
    assert_ne!(map.get("nine"), Some(&31415));

    assert!(map.get("foo").is_none());
    assert!(map.get("bar").is_none());
    assert!(map.get("faz").is_none());

    assert_eq!(map.len, 3);
    assert_eq!(map.remove("pi"), Some(31415));

    assert_eq!(map.len, 2);
    assert_eq!(map.remove("pi"), None);
}
