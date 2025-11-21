#![allow(dead_code)]
mod map_matrix;
mod table_matrix;
mod basic;
pub mod alloc;
use std::{collections::{HashMap}};
pub use crate::{basic::{Matrix, MatrixInfo, Pair}, map_matrix::{HashMapStore, MapMatrix, TreeStore}};

// Type aliases para facilitar o uso das diferentes implementações de matrizes

/// Matriz baseada em HashMap
pub type HashMapMatrix = MapMatrix<HashMapStore<Pair, f64>, HashMapStore<usize, Vec<(Pair, f64)>>>;
/// Matriz baseada em BTreeMap
pub type TreeMatrix = MapMatrix<TreeStore<Pair, f64>, TreeStore<usize, Vec<(Pair, f64)>>>;
/// Matriz baseada em tabela (vetor de vetores)
pub type TableMatrix = table_matrix::TableMatrix;

/// Epsilon para comparações de ponto flutuante
pub const EPSILON : f64 = 1e-8;



fn info_eq(expected: &MatrixInfo, current: &MatrixInfo) -> bool {
    if expected.size != current.size {
        return false;
    }
    let mut exp_map = HashMap::new();
    for (pos, value) in expected.values.iter() {
        exp_map.insert(pos, value);
    }
    for (pos, value) in current.values.iter() {
        match exp_map.get(pos) {
            Some(v) => {
                if (*v - value).abs() > EPSILON {
                    return false;
                }
            },
            None => return false,
        }
    }
    true
}

fn diff(expected: &MatrixInfo, current: &MatrixInfo) -> Vec<(Pair, (Option<f64>, Option<f64>))> {
    let mut exp_map = HashMap::new();
    for (pos, value) in expected.values.iter() {
        exp_map.insert(pos, value);
    }
    let mut diff = Vec::new();
    for (pos, value) in current.values.iter() {
        match exp_map.get(pos) {
            Some(v) => {
                if (*v - value).abs() > EPSILON {
                    diff.push((*pos, (Some(**v), Some(*value))));
                }
            },
            None => diff.push((*pos, (None, Some(*value)))),
        }
    }
    diff
}
fn mul<M :  Matrix>(ainfo: &MatrixInfo, binfo: &MatrixInfo)  -> MatrixInfo {
    let a = M::from_info(ainfo).transposed();
    let b = M::from_info(binfo).transposed();
    M::mul(&b, &a).to_info()
    

}
