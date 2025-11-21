use std::{borrow::Cow};

use crate::{basic::Pair, map_matrix::Map};

#[derive(Clone)]
pub struct TransposableMap<M : Map<Pair, f64>> {
	map: M,
	transposed: bool
}

impl<M : Map<Pair, f64>> TransposableMap<M>  {
	pub  fn new(map: M) -> Self {
		TransposableMap {
			map,
			transposed: false
		}
	}
	pub fn transpose(&mut self) {
		self.transposed = !self.transposed;
	}
}
impl<M : Map<Pair, f64>> Map<Pair, f64> for TransposableMap<M> {
	fn from_iter<I: IntoIterator<Item=(Pair,f64)>>(iter: I) -> Self {
		TransposableMap {
			map: M::from_iter(iter),
			transposed: false
		}
	}

	fn set_or_insert(&mut self, key: Pair, value: f64) {
		if self.transposed {
			self.map.set_or_insert((key.1, key.0), value);
		} else {
			self.map.set_or_insert(key, value);
		}
	}

	fn remove(&mut self, key: &Pair) {
		if self.transposed {
			self.map.remove(&(key.1, key.0));
		} else {
			self.map.remove(key);
		}
	}

	fn get(&self, key: &Pair) -> Option<&f64> {
		if self.transposed {
			self.map.get(&(key.1, key.0))
		} else {
			self.map.get(key)
		}
	}

	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(Pair, Cow<'a, f64>)> + 'a> {
		if self.transposed {
			Box::new(self.map.iter()
				.map(|(pos, value)| {
					((pos.1, pos.0) , value) 
				}))
		} else {
			self.map.iter()
		}
	}
	fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=(Pair, &'a mut f64)> + 'a> {
		if self.transposed {
			Box::new(self.map.iter_mut()
				.map(|(pos, value)| {
					((pos.1, pos.0) , value) 
				}))
		} else {
			self.map.iter_mut()
		}
	}
}

