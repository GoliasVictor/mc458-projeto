pub type Pair = (usize, usize); 
pub trait Matrix : From<MatrixInfo> + Into<MatrixInfo> {
    fn new(size: Pair) -> Self;
    fn set(&mut self, pos: Pair, value: f64);
    fn get(&self, pos: Pair) -> f64;
    fn transposed(self) -> Self;
    fn add(a : &Self, b : &Self) -> Self;
    fn mul(a : &Self, b : &Self) -> Self;
}
#[derive(Clone)]
pub struct MatrixInfo {
    pub size: Pair,
    pub values: Vec<(Pair, f64)>
}

impl MatrixInfo {
	pub fn print_values(&self) {
		for (pos, value) in self.values.iter() {
			println!("{:?} = {}", pos, value);
		}
	}
}