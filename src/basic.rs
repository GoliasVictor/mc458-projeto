
pub type Pair = (usize, usize); 

pub trait Matrix {
    
    fn new(size: Pair) -> Self;
    fn set(&mut self, pos: Pair, value: f64);
    fn get(&self, pos: Pair) -> f64;
    fn transposed(self) -> Self;
    fn add(a : &Self, b : &Self) -> Self;
    fn mul(a : &Self, b : &Self) -> Self;
    fn muls(a : &Self, scalar: f64) -> Self;
	fn from_info(info: &MatrixInfo) -> Self;
	fn to_info(&self) -> MatrixInfo;
}

#[derive(Clone)]
/// Metadata for a matrix: its dimensions and the stored entries.
///
/// `MatrixInfo` collects the essential information needed to describe a matrix:
/// - `size`: a `Pair` describing the matrix dimensions (for example, row and column counts).
/// - `values`: a `Vec<(Pair, f64)>` holding entries as `(position, value)`, where `position` is a `Pair` (row, column).
pub struct MatrixInfo {
    /// 
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