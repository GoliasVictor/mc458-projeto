//#![allow(unused)]
mod matrix_generator;
use std::{
    fmt::Display,
    hint::black_box,
    rc::Rc,
    time::{Duration, Instant},
};

use matrix_generator::MatrixGenerator;
use projeto::{HashMapMatrix, Matrix, Pair, TableMatrix, TreeMatrix};
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

fn mul<T: Matrix>(a: &T, b: &T) -> T {
    black_box(T::mul(a, b))
}
fn add<T: Matrix>(a: &T, b: &T) -> T {
    black_box(T::add(a, b))
}

fn transposed<T: Matrix>(a: T) -> T {
    black_box(a.transposed())
}
fn muls<T: Matrix>(a: T, scalar: f64) -> T {
    black_box(T::muls(&a, scalar))
}
fn get<T: Matrix>(a: T, pos: Pair) -> T {
    black_box(a.get(pos));
    a
}
fn set<T: Matrix>(mut a: T, pos: Pair, value: f64) -> T {
    black_box(a.set(pos, value));
    a
}
trait Cross<A: Clone>: Iterator<Item = A> {
    fn cross<'a, B, IB>(self, ib: IB) -> impl Iterator<Item = (A, B)>
    where
        B: Clone,
        IB: Iterator<Item = B> + Clone;
}

impl<A: Clone, IA: Iterator<Item = A>> Cross<A> for IA {
    fn cross<'a, B, IB>(self, ib: IB) -> impl Iterator<Item = (A, B)>
    where
        B: Clone,
        IB: Iterator<Item = B> + Clone,
    {
        self.flat_map(move |a| ib.clone().map(move |b| (a.clone(), b.clone())))
    }
}
fn cross<'a, A, IA, B, IB>(ia: IA, ib: IB) -> impl Iterator<Item = (A, B)>
where
    A: Clone,
    B: Clone,
    IA: Iterator<Item = A>,
    IB: Iterator<Item = B> + Clone,
{
    ia.flat_map(move |a| ib.clone().map(move |b| (a.clone(), b.clone())))
}
type MatrixGen<M> = Box<dyn Fn() -> M>;
type BuilderMatrixGen<M> = Rc<dyn Fn(usize, usize) -> MatrixGen<M>>;
type Operation<M> = Rc<dyn Fn(&M, &M) -> M>;

struct Id<'a>(&'a str, u32, i32, &'a str, &'a str, &'a str);

impl<'a> Display for Id<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}
#[derive(Serialize, Deserialize)]
struct Record {
    matrix_type: String,
    population: usize,
    occupation: usize,
    size: usize,
    operation: String,
    durations: Vec<Duration>,
}
struct Records {
    records: Vec<Record>,
}
impl Records {
    fn add_record(&mut self, record: Record) {
        let mean = record.durations.iter().sum::<Duration>().as_millis() as f64
            / (record.durations.len() as f64);

        println!(
            "{}, {}, {:0.2}, {}, {}, {}, {:?}",
            record.matrix_type,
            record.size,
            record.population,
            record.operation,
            record.durations.len(),
            mean,
            record
                .durations
                .iter()
                .map(|d| d.as_millis() as f64)
                .map(|d| (d - mean).powf(2.0))
                .sum::<f64>()
                / (record.durations.len() as f64)
        );
        self.records.push(record);
    }
}

#[derive(Serialize, Deserialize)]
struct ExponentialRecord {
    matrix_type: String,
    i: usize,
    population: usize,
    operation: String,
    durations: Vec<Duration>,
}

fn get_density(i : u32) -> Vec<f64> { 
    if i < 4 {
        vec![1.0 / 100.0, 5.0 / 100.0, 10.0 / 100.0, 20.0 / 100.0]
    } else {
        vec![
            1.0 / 10.0_f64.powi(i as i32),
            1.0 / 10.0_f64.powi(i as i32 + 1),
            1.0 / 10.0_f64.powi(i as i32 + 2),
        ]
    }
}
fn exponential_benchs<M: Matrix>(name: &str, records: &mut Vec<ExponentialRecord>, max_expoent : u32) {
    let bin_operations: [(&str, Operation<M>); 2] = [
        ("mul", Rc::new(|a, b| mul::<M>(a, b))),
        ("add", Rc::new(|a, b| add::<M>(a, b))),
    ];
    let unary_operations: [(&str, Rc<dyn Fn(M, Pair, f64) -> M>); 4] = [
        ("transpose", Rc::new(|a, _pos, _s| transposed::<M>(a))),
        ("muls", Rc::new(|a, _pos, s| muls::<M>(a, s))),
        ("get", Rc::new(|a, pos, _s| get::<M>(a, pos))),
        ("set", Rc::new(|a, pos, s| set::<M>(a, pos,s))),
    ];
    let max_duration = Duration::from_secs(1);
    let max_iterations = 20;
    let min_iterations = 1;

    for (op_name, op) in bin_operations.iter() {
        for i in 1..=max_expoent {
            let len = 10usize.pow(i);
            let densities = get_density(i);
            for den in densities {
                let population = (den * (len * len) as f64) as usize;
                let mut j = 0;
                let start_bench = Instant::now();
                let mut durations = Vec::new();
                while (j < min_iterations || Instant::now()  - start_bench < max_duration) && j < max_iterations {
                    let a = MatrixGenerator::uniform::<M>((len, len), population);
                    let b = MatrixGenerator::uniform::<M>((len, len), population);
                    let start = Instant::now();
                    let c = black_box(op(black_box(&a), black_box(&b)));
                    let duration = Instant::now() - start;
                    drop(black_box(c));
                    j += 1;
                    durations.push(duration);
                }
                println!("{}, {}, {}, {:?}, {}", name, i, population, durations.iter().sum::<Duration>().div_f64(durations.len() as f64), durations.len());
                records.push(ExponentialRecord {
                    matrix_type: name.to_string(),
                    operation: op_name.to_string(),
                    i: i as usize,
                    population,
                    durations,
                });
            }
        }
    }
    let mut rand = rand::rng();
    for (op_name, op) in unary_operations.iter() {
        for i in 1..=max_expoent {
            let len = 10usize.pow(i);
            let densities = get_density(i);
            for den in densities {
                let population = (den * (len * len) as f64) as usize;
                let mut j = 0;
                let start_bench = Instant::now();
                let mut durations = Vec::new();
                while (j < min_iterations || Instant::now()  - start_bench < max_duration) && j < max_iterations {
                    let a = MatrixGenerator::uniform::<M>((len, len), population);
                    let pos = (
                        rand.random_range(0..len),
                        rand.random_range(0..len),
                    );
                    let scalar = rand.random_range(-10.0..10.0);

                    let start = Instant::now();
                    black_box(op(black_box(a), black_box(pos), black_box(scalar)));
                    let duration = Instant::now() - start;
                    
                    j += 1;
                    durations.push(duration);
                }
                println!("{}, {}, {}, {:?}, {}", name, i, population, durations.iter().sum::<Duration>().div_f64(durations.len() as f64), durations.len());
                records.push(ExponentialRecord {
                    matrix_type: name.to_string(),
                    operation: op_name.to_string(),
                    i: i as usize,
                    population,
                    durations,
                });
            }
        }
    }
}

fn bench_matrix<M: Matrix>(name: &str, records: &mut Records, qt_samples: usize) {
    let occupation_percentage: [i32; 4] = [1, 5, 10, 20]; //1] = [1]; //

    let bin_operations: [(&str, Operation<M>); 2] = [
        ("mul", Rc::new(|a, b| mul::<M>(a, b))),
        ("add", Rc::new(|a, b| add::<M>(a, b))),
    ];
    let unary_operations: [(&str, Rc<dyn Fn(M, Pair, f64) -> M>); 4] = [
        ("transpose", Rc::new(|a, _pos, _s| transposed::<M>(a))),
        ("muls", Rc::new(|a, _pos, s| muls::<M>(a, s))),
        ("get", Rc::new(|a, pos, _s| get::<M>(a, pos))),
        ("set", Rc::new(|a, pos, s| set::<M>(a, pos,s))),
    ];
    let min = 10.0;
    let max = 500.0;
    let step = (max - min) / (qt_samples as f64);
    let mut rand = rand::rng();
    let mut lens = (0..qt_samples)
        .rev()
        .map(|i| (min + step * i as f64) as usize)
        .collect::<Vec<_>>();

    lens.shuffle(&mut rand);

    let iter = bin_operations
        .iter()
        .cross(occupation_percentage.iter().cloned())
        .cross(lens.clone().into_iter())
        .map(|((nop, occupation), i)| (i, occupation, nop));

    for (len, occupation, nop) in iter {
        let size = (len, len);
        let density = (occupation as f64) / 100.0;
        let population = (density * (len * len) as f64) as usize;
        let (op_name, op) = nop;
        let mut durations = Vec::new();
        let a = MatrixGenerator::uniform::<M>(size, population);
        let b = MatrixGenerator::uniform::<M>(size, population);
        let start = Instant::now();
        let c = black_box(op(black_box(&a), black_box(&b)));
        let duration = Instant::now() - start;
        durations.push(duration);
        drop(c);

        records.add_record(Record {
            matrix_type: name.to_string(),
            population: population,
            occupation: occupation as usize,
            size: len,
            operation: op_name.to_string(),
            durations,
        });
    }



    let iter = unary_operations
        .iter()
        .cross(occupation_percentage.iter().cloned())
        .cross(lens.into_iter())
        .map(|((nop, occupation), i)| (i, occupation, nop));
    for (len, occupation, nop) in iter {
        let size = (len, len);
        let density = (occupation as f64) / 100.0;
        let population = (density * (len * len) as f64) as usize;
        let (op_name, op) = nop;
        let mut durations = Vec::new();
        let a = black_box(MatrixGenerator::uniform::<M>(size, population));
        let pos = (
            rand.random_range(0..size.0),
            rand.random_range(0..size.1),
        );
        let scalar = rand.random_range(-10.0..10.0);
        let start = Instant::now();
        let c = black_box(op(a, black_box(pos), black_box(scalar)));
        let duration = Instant::now() - start;
        drop(black_box(c));
        durations.push(duration);

        records.add_record(Record {
            matrix_type: name.to_string(),
            population: population,
            occupation: occupation as usize,
            size: len,
            operation: op_name.to_string(),
            durations,
        });
    }
}

pub fn b2(){
    let mut records = Vec::new();
    exponential_benchs::<TableMatrix>("TableMatrix", &mut records, 3);
    exponential_benchs::<HashMapMatrix>("HashMapMatrix", &mut records, 6);
    exponential_benchs::<TreeMatrix>("TreeMatrix", &mut records, 6);
    let file = fs::File::create(format!("b2.json")).unwrap();
    serde_json::to_writer_pretty(file, &records).unwrap();
}
pub fn b1(){
    let mut records = Records {
        records: Vec::new(),
    };
    bench_matrix::<HashMapMatrix>("HashMapMatrix", &mut records, 100);
    bench_matrix::<TreeMatrix>("TreeMatrix", &mut records, 100);
    bench_matrix::<TableMatrix>("TableMatrix", &mut records, 100);
    let file = fs::File::create(format!("b1.json")).unwrap();
    serde_json::to_writer_pretty(file, &records.records).unwrap();
}

pub fn criterion_benchmark() {
    b1();
    b2();
}

pub fn main() {
    criterion_benchmark();
}
