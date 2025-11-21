use std::collections::HashSet;
use rand::{Rng, rngs::ThreadRng};
use projeto::{Matrix, MatrixInfo, Pair};
use rand::seq::SliceRandom;

use rand::seq::IndexedRandom;
pub struct MatrixGenerator;
impl MatrixGenerator {
    pub fn uniform<M : Matrix>(size: Pair, population: usize) -> M {
        let mut rng = rand::rng();
        let mut values = Vec::new();
        let total_elements = size.0 * size.1;
        let non_zero_elements = population.min(total_elements);

        let mut positions = Vec::new();

    
        let samples = rand::seq::index::sample(&mut rng, total_elements, non_zero_elements);
        
        positions = samples.iter()
            .map(|index| {
                (index % size.0, index / size.0)
            })
            .collect();

        for &(row, col) in positions.iter() {
            let value: f64 = rng.random_range(-10.0..10.0);
            values.push(((row, col), value));
        }
        M::from_info(
            &MatrixInfo {
                size,
                values,
            }   
        )
    }
    pub fn permutation_matrix<M : Matrix>(size: Pair, population : usize) -> M {
        let mut rng = rand::rng();
        let total = size.0 * size.1;
        let mut rows = (0..size.0).collect::<Vec<usize>>();
        let mut cols = (0..size.1).collect::<Vec<usize>>();
        rows.shuffle(&mut rng);
        cols.shuffle(&mut rng);
        let values = rows
            .iter()
            .zip(cols.iter())
            .take(std::cmp::min(size.0, size.1))
            .map(|(&r, &c)| ((r, c), 1.0))
            .collect();
        M::from_info(
            &MatrixInfo {
                size,
                values,
            }   
        )
    }
}
