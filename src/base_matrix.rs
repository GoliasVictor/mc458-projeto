use crate::basic::{Matrix, Pair, MatrixInfo};
use std::collections::HashMap;

pub struct SimpleMatrix {
    size: Pair,
    values: HashMap<Pair, f64>
}
impl Into<MatrixInfo> for SimpleMatrix {
    fn into(self) -> MatrixInfo {
        let mut values = Vec::new();
        for (pos, value) in self.values {
            values.push((pos, value));
        }
        MatrixInfo {
            size: self.size,
            values
        }
    }
}
impl From<MatrixInfo> for SimpleMatrix {
    fn from(info: MatrixInfo) -> Self {
		let mut matrix = SimpleMatrix {
			size: info.size,
			values: HashMap::new(),
		};
		for (pos, value) in info.values {
			matrix.set(pos, value);
		}
		matrix
    }
}
impl Matrix for SimpleMatrix {
	fn new(size: Pair) -> SimpleMatrix{
		SimpleMatrix {
			size,
			values: HashMap::new(),
		}
	}

	fn set(&mut self, pos: Pair, value: f64) {
        if value == 0.0 {
            self.values.remove(&pos);
        } else {
            self.values.insert(pos, value);
        }
    }
    fn get(&self, pos: Pair) -> f64 {
        *self.values.get(&pos).unwrap_or(&0.0)
    }

    fn transposed(self) -> SimpleMatrix {
        let mut c = SimpleMatrix {
            size: (self.size.1, self.size.0), 
            values: HashMap::new()
        };
        for (pos, va) in self.values.iter()  {
            let pos = *pos;
            c.set((pos.1, pos.0), *va);
        }
        return c
    }
    fn add(a : &SimpleMatrix, b : &SimpleMatrix) -> SimpleMatrix {
        let mut c = SimpleMatrix {
            size: a.size, 
            values: HashMap::new()
        };
        for (pos, va) in a.values.iter()  {
            let pos = *pos;
            let vb = b.get(pos);

            c.set(pos, vb + va);
        }
        return c
    }
    fn mul(a : &SimpleMatrix, b : &SimpleMatrix) -> SimpleMatrix {
        let mut c = SimpleMatrix {
            size: (a.size.0, b.size.1), 
            values: HashMap::new()
        };
        for (apos, va) in a.values.iter()  {
            for (bpos, vb) in b.values.iter()  {
                if apos.1 == bpos.0 {
                    let pos = (apos.0, bpos.1);
                    let value =  c.get(pos)+ vb*va;
                    c.set(pos, value);
                }
            }
        }
        return c;
    }
}

