#![allow(unused)]
mod matrix_generator;
use std::{any::TypeId, cell::{Cell, RefCell}, fmt::Display, hint::black_box, rc::Rc, sync::Arc, time::{Duration, Instant}};

use criterion::{Bencher, BenchmarkId, Criterion, criterion_group, criterion_main};
use matrix_generator::MatrixGenerator;
use projeto::{
    HashMapMatrix, TreeMatrix,
    Matrix, MatrixInfo, Pair,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

fn mul<T: Matrix>(a: T, b: T) -> () {
    T::mul(&a, &b);
}
fn add<T: Matrix>(a: T, b: T) -> () {
    T::add(&a, &b);
}
trait Cross<A : Clone> : Iterator<Item = A>{
	fn cross<'a, B, IB>(self , ib : IB) -> impl Iterator<Item=(A, B)>
	where 
		B : Clone,
		IB : Iterator<Item=B> + Clone;
}

impl<A : Clone, IA : Iterator<Item = A>> Cross<A>  for IA
{
	fn cross<'a, B, IB>(self , ib : IB) -> impl Iterator<Item=(A, B)>
	where 
		B : Clone,
		IB : Iterator<Item=B> + Clone,
	{
		self.flat_map(move |a| ib.clone().map(move |b| (a.clone(), b.clone())))
	}
}
fn cross<'a, A, IA, B, IB>(ia : IA, ib : IB) -> impl Iterator<Item=(A, B)> 
where 
	A : Clone,
	B : Clone,
	IA : Iterator<Item=A>,
	IB : Iterator<Item=B> + Clone,
{
	ia.flat_map(move |a| ib.clone().map(move |b| (a.clone(), b.clone())))
}
type MatrixGen<M> = Box<dyn Fn() -> M>;
type BuilderMatrixGen<M> = Rc<dyn Fn(usize, usize) ->MatrixGen<M>>;
type Operation<M> = Rc<dyn Fn(M, M) -> ()>;


struct Id<'a>(&'a str, u32, i32, &'a str, &'a str, &'a str);

impl<'a> Display for Id<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{};{};{};{};{};{}", 
			self.0, 
			self.1,
			self.2,
			self.3,
			self.4,
			self.5
		)
	}
}
#[derive(Serialize, Deserialize)]
struct Record {
	matrix_type : String,
	population : usize,
	size : usize,
	operation : String,
	generator : String,
	durations : Vec<Duration>
}
struct Records {
	records : Vec<Record>
}
impl Records {
	fn add_record(&mut self, record : Record) {
		let mean = record.durations.iter()
			.sum::<Duration>()
			.as_millis() as f64 / (record.durations.len() as f64);
			
		
		println!("{}, {}, {:0.2}, {}, {}, {}, {:?}, {}", 
			record.matrix_type,
			record.size,
			record.population,
			record.generator,
			record.operation,
			record.durations.len(),
			mean, 
			record.durations.iter()
				.map(|d| d.as_millis() as f64 )
				.map(|d| (d - mean).powf(2.0))
				.sum::<f64>() / (record.durations.len() as f64)
		);
		self.records.push(record);
	}
}

#[derive(Serialize, Deserialize)]
struct ExponentialRecord {
	matrix_type : String,
	i : usize,
	population : usize,
	duration : Duration,
}
fn exponential_benchs<M : Matrix>(name : &str) {
	let mut records : Vec<ExponentialRecord> = Vec::new();
	for i in 1..6 { 
		let len = 10usize.pow(i); 
		let densities  = if i < 4 {
			vec![1.0/100.0, 5.0/100.0, 10.0/100.0, 20.0/100.0]
		} else {
			vec![
				1.0/10.0_f64.powi(i as i32), 
				1.0/10.0_f64.powi(i as i32 + 1), 
				1.0/10.0_f64.powi(i as i32 + 2), 
			]
		};
		for den in densities {
			let population = (den * (len * len) as f64) as usize;
			let a = MatrixGenerator::uniform::<M>((len, len), population);
			let b = MatrixGenerator::uniform::<M>((len, len), population);
			let start = Instant::now();
			black_box(mul::<M>(black_box(a), black_box(b)));
			let duration = Instant::now() - start;
			records.push(ExponentialRecord {
				matrix_type : std::any::type_name::<M>().to_string(),
				i: i as usize,
				population,
				duration,
			});
			println!("{}, {}, {}, {:?}", 
				name,
				i,
				population,
				duration
			);
		}
	}
}

fn bench_matrix<M : Matrix>(name: &str) {
	let mut records = Records {
		records: Vec::new(),
	};

	let occupation_percentage: [i32;4] = [1, 5, 10, 20]; //1] = [1]; //

	let uniform : BuilderMatrixGen<M>  = Rc::new(|len, population| {
		Box::new(move || MatrixGenerator::uniform::<M>((len, len), population))
	});
	
	let permutation : BuilderMatrixGen<M>  = Rc::new(|len, population| {
		Box::new(move || MatrixGenerator::permutation_matrix::<M>((len, len), len / 10))
	});
	let generators : [(&str, BuilderMatrixGen<M>); 1] = [
			("uniform", uniform),
	];
	let operations : [(&str, Operation<M>); 2] = [
		("mul", Rc::new(|a, b| mul::<M>(a, b))),
		("add", Rc::new(|a, b: M| add::<M>(a, b)))
	];
	let qt_samples = 1000;
	let step = 2;
	let min = 10;
	let mut rand = rand::rng();
	let mut lens = (0..qt_samples).rev()
		.map(|i|  (min + step * i)).collect::<Vec<_>>();

	lens.shuffle(&mut rand);

	let iter = operations.iter()
		.cross(occupation_percentage.iter().cloned())
		.cross(generators.iter().cloned())
		.cross(lens.into_iter())
		.map(|(((nop, occupation), lb), i)| {
			(i, occupation, lb, nop)
		});
	
	for (len, occupation, lb, nop) in iter {
		let size = (len, len);
		let density  = (occupation as f64) / 100.0;
		let mut population = (density * (len * len) as f64) as usize;
		let (op_name, op) = nop;
		let (gen_name, genm) = lb;
		if gen_name == "permutation" {
			population = len;
		}
		let gen_a = genm(len, population);
		let gen_b = genm(len, population);
		let mut j = 0;
		let start_bench = Instant::now();
			
			let (a, b) = (gen_a(), gen_b());
			let mut durations = Vec::new();
				let a = M::from_info(&a.to_info());
				let b = M::from_info(&b.to_info());
				let start = Instant::now();
				black_box(op(black_box(a), black_box(b)));
				let duration =  Instant::now() - start;
				j += 1;
				durations.push(duration);

			records.add_record(Record {
				matrix_type: name.to_string(),
				population: population,
				size: len,
				generator : gen_name.to_string(),
				operation: op_name.to_string(),
				durations,
			});


	}

	let file = fs::File::create(format!("{}.json", name)).unwrap();
	serde_json::to_writer_pretty(file, &records.records).unwrap();
}
pub fn criterion_benchmark() {
    bench_matrix::<HashMapMatrix>("HashMapMatrix");
	bench_matrix::<TreeMatrix>("TreeMatrix");
}

pub fn main(){
	criterion_benchmark();
}


fn adds(mut a : u32, b : u32) -> u32 {
	let diff = loop {
		if a < b {
			break a + b;
		}
		a -= 1;
	};
	diff
}