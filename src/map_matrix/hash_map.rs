use crate::map_matrix::{Map, MapVec};

/// https://docs.rs/hashbrown/latest/src/hashbrown/raw/mod.rs.html#1496-1524
/// https://docs.rs/hashbrown/latest/src/hashbrown/raw/mod.rs.html#103-160

use std::{borrow::Cow, collections::HashMap, hash::Hash};
pub struct HashMapStore<K :Copy + Eq + Hash, V> {
	values: HashMap<K, V>,
}
impl<K : Copy + Eq + Hash, V : Clone> Map<K, V> for HashMapStore<K, V> {
	fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
		let values : Vec<(K, V)> = iter.into_iter().collect();

		HashMapStore {
			values: HashMap::from_iter(values.into_iter()),
		}
	}
	fn set_or_insert(&mut self, key: K, value: V) {
		self.values.insert(key, value);
	}
	fn remove(&mut self, key: &K) {
		self.values.remove(key);
	}
	fn get(&self, key: &K) -> Option<&V> {
		self.values.get(key)
	}
	
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(K, Cow<'a, V>)> + 'a> {
		Box::new(self.values.iter()
			.map(|(k, v)| (*k, Cow::Borrowed(v))) )
	}
} 


impl <K : Copy + Eq + Hash, U : Clone> MapVec<K, U> for HashMapStore<K, Vec<U>> {
	fn add_to_vec(&mut self, key: K, value: U) {
		self.values.entry(key)
			.or_insert_with(Vec::new)
			.push(value);
	}
}
