#![allow(dead_code)]
mod base_matrix;
mod map_matrix;
mod basic;
mod alloc;
use base_matrix::SimpleMatrix;
use std::{collections::{HashMap, HashSet}};
use rand::prelude::*;
use crate::{basic::{Matrix, MatrixInfo, Pair}, map_matrix::{HashMapStore, MapMatrix, TreeStore}};



const EPSILON : f64 = 1e-8;
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
type HashMapMatrix = MapMatrix<HashMapStore<Pair, f64>, HashMapStore<usize, Vec<(Pair, f64)>>>;
type TreeMatrix = MapMatrix<TreeStore<Pair, f64>, TreeStore<usize, Vec<(Pair, f64)>>>;

struct RandGenerator {
    rng: ThreadRng,
}

impl RandGenerator {
    fn new() -> Self {
        RandGenerator {
            rng: rand::rng(),
        }
    }

    fn gen_matrix_info(&mut self, size: Pair, density: f64) -> MatrixInfo {
        let mut values = Vec::new();
        let total_elements = size.0 * size.1;
        let non_zero_elements = (total_elements as f64 * density) as usize;

        let mut positions = HashSet::new();
        while positions.len() < non_zero_elements {
            let row = self.rng.random_range(0..size.0);
            let col = self.rng.random_range(0..size.1);
            positions.insert((row, col));
        }

        for &(row, col) in positions.iter() {
            let value: f64 = self.rng.random_range(1.0..10.0);
            values.push(((row, col), value));
        }

        MatrixInfo {
            size,
            values,
        }
    }
}
fn main() {
    let mut rng = RandGenerator::new();
    loop {
        let ainfo = rng.gen_matrix_info((10, 10), 1.0);
        let binfo = rng.gen_matrix_info((10, 10), 1.0);
        println!("Matrix SimpleMatrix:");
        let res1=  mul::<SimpleMatrix>(&ainfo, &binfo);
        println!("Matrix HashMapMatrix:");
        let res2 = mul::<HashMapMatrix>(&ainfo, &binfo);
        println!("Matrix TreeMatrix:");
        let res3 = mul::<TreeMatrix>(&ainfo, &binfo);
        
        let cop2 = info_eq(&res1, &res2);
        let cop3 = info_eq(&res1, &res3); 
        println!("{}", cop2);
        println!("{}", cop3);
        println!();
        if !cop2  {
            println!("Diff Simple vs HashMap:");
            let diffs = diff(&res1, &res2);
            for (pos, (v1, v2)) in diffs {
                println!("{:?}: ({:?}, {:?})", pos, v1, v2);
            }   
            break;
        }
        break;
    }
    
}
