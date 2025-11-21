use crate::{basic::{Matrix, MatrixInfo, Pair}};

#[derive(Clone, Debug)]
pub struct TableMatrix {
	pub size: Pair,
	pub data: Vec<Vec<f64>>,
}

impl TableMatrix {
	fn zero_like(&self) -> Self {
		TableMatrix::new(self.size)
	}
}

impl Matrix for TableMatrix {
	fn new(size: Pair) -> Self {
		TableMatrix {
			size,
			data: vec![vec![0.0; size.1]; size.0],
		}
	}
	fn from_info(info: &MatrixInfo) -> Self {
		let mut m = TableMatrix::new(info.size);
		for (pos, value) in info.values.iter() {
			let (r, c) = *pos;
			m.data[r][c] = *value;
		}
		m
	}

	fn to_info(&self) -> MatrixInfo {
		let mut values = Vec::new();
		for i in 0..self.size.0 {
			for j in 0..self.size.1 {
				let v = self.data[i][j];
				values.push(((i, j), v));
			}
		}
		MatrixInfo {
			size: self.size,
			values: values,
		}
	}

	fn transposed(self) -> Self {
		let mut t = TableMatrix::new((self.size.1, self.size.0));
		for i in 0..self.size.0 {
			for j in 0..self.size.1 {
				t.data[j][i] = self.data[i][j];
			}
		}
		t
	}
	fn muls(a : &Self, scalar: f64) -> Self {
		let n = a.size;
		let mut res = TableMatrix::new(n);
		for i in 0..n.0 {
			for j in 0..n.1 {
				res.data[i][j] = a.data[i][j] * scalar;
			}
		}
		res
	}
	fn mul(a: &Self, b: &Self) -> Self {
		assert_eq!(a.size, b.size);
		let n = a.size;
		let mut res = TableMatrix::new(n);
		for i in 0..n.0 {
			for k in 0..n.1 {
				let aik = a.data[i][k];
				for j in 0..n.1 {
					res.data[i][j] += aik * b.data[k][j];
				}
			}
		}
		res
	}
	
	
	fn set(&mut self, pos: Pair, value: f64) {
		self.data[pos.0][pos.1] = value;
	}
	
	fn get(&self, pos: Pair) -> f64 {
		self.data[pos.0][pos.1]
	}
	
	fn add(a : &Self, b : &Self) -> Self {
		assert_eq!(a.size, b.size);
		let n = a.size;
		let mut res = TableMatrix::new(n);
		for i in 0..n.0 {
			for j in 0..n.1 {
				res.data[i][j] = a.data[i][j] + b.data[i][j];
			}
		}
		res
	}
}