mod tree_map;
mod hash_map;
mod transposable_map;
pub use hash_map::HashMapStore;
pub use tree_map::TreeStore;
use transposable_map::TransposableMap;
use crate::basic::{Matrix, MatrixInfo, Pair};
use core::panic;
use std::borrow::Cow; 
pub trait Map<K : Copy, U : Clone >: {
	fn from_iter<I: IntoIterator<Item=(K,U)>>(iter: I) -> Self;
	fn set_or_insert(&mut self, key: K, value: U);
	fn remove(&mut self, key: &K);
	fn get(&self, key: &K) -> Option<&U>;
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(K, Cow<'a, U>)> + 'a>;
}

pub trait MapVec <K : Copy, U : Clone> : Map<K, Vec<U>> { 
	fn add_to_vec(&mut self, key: K, value: U);
}


pub struct MapMatrix <T:  Map<Pair, f64>, LM : MapVec<usize, (Pair, f64)>> {
    size: Pair,
    values: TransposableMap<T>,
	phatom: std::marker::PhantomData<LM>
}

impl<T:  Map<Pair, f64>, LM : MapVec<usize, (Pair, f64)>> Matrix for MapMatrix<T, LM> {
	fn new(size: Pair) -> MapMatrix<T, LM>{
		MapMatrix {
			size,
			values: TransposableMap::new(T::from_iter(std::iter::empty())),
			phatom: std::marker::PhantomData
		}
	}
	fn set(&mut self, pos: Pair, value: f64) {
        if value == 0.0 {
            self.values.remove(&pos);
        } else {
            self.values.set_or_insert(pos, value);
        }
    }
    fn get(&self, pos: Pair) -> f64 {
        *self.values.get(&pos).unwrap_or(&0.0)
    }

    fn transposed(mut self) -> MapMatrix<T, LM> {
		self.size = (self.size.1, self.size.0);
        self.values.transpose();
		self
    }
    fn add(a : &MapMatrix<T, LM>, b : &MapMatrix<T, LM>) -> MapMatrix<T, LM> {
        let mut c = MapMatrix::new(a.size);
        for (pos, va) in a.values.iter()  {
            c.set(pos, *va);
        }
		for (pos, vb) in b.values.iter()  {
			let value =  c.get(pos)+ *vb;
			c.set(pos, value);
		}
        return c
    }
    fn mul(a : &MapMatrix<T, LM>, b : &MapMatrix<T, LM>) -> MapMatrix<T, LM> {
        let mut c = MapMatrix::new((a.size.0, b.size.1));
		let mut acolumns = LM::from_iter(std::iter::empty()); 
		let mut brows = LM::from_iter(std::iter::empty());
		for (apos, va) in a.values.iter()  {
			acolumns.add_to_vec(apos.1, (apos, *va));
		}
		for (bpos, vb) in b.values.iter() {
			brows.add_to_vec(bpos.0, (bpos, *vb));
		}
		for (i, avalues) in acolumns.iter() {
			let Some(bvalues) = brows.get(&i) else {
				continue;
			};
			for (apos, va) in avalues.iter()  {
				for (bpos, vb) in  bvalues.iter() {
					if apos.1 == bpos.0 {
						let pos = (apos.0, bpos.1);
						let value =  c.get(pos)+ vb*va;
						c.set(pos, value);
					}
					else {
						panic!("Incompatible matrices for multiplication");
					}
				}
			}
		}
        return c;
    }

	fn to_info(&self) -> MatrixInfo {
		let mut values = Vec::new();
		for (pos, value) in self.values.iter() {
			values.push(( pos, value.into_owned()));
		}
		MatrixInfo {
			size: self.size,
			values
		}
	}
	fn from_info(info: &MatrixInfo) -> Self {
		MapMatrix {
			size: info.size,
			values: TransposableMap::new(T::from_iter(info.values.iter().map(|(pos, value)| (*pos, *value)))),
			phatom: std::marker::PhantomData
		}
	}
}