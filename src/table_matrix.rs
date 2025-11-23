//! Implementação de matriz densa usando vetores.
//!
//! Este módulo fornece `TableMatrix`, uma implementação de matriz que armazena
//! todos os valores (incluindo zeros) em um vetor de vetores (`Vec<Vec<f64>>`).
//!
//! # Características
//!
//! - **Acesso**: O(1) para qualquer elemento
//! - **Memória**: O(n²) onde n é a dimensão da matriz
//! - **Melhor uso**: Matrizes densas ou pequenas onde a maioria dos elementos é não-zero
//!
//! # Desvantagens
//!
//! - Alto uso de memória para matrizes grandes e esparsas
//! - Armazena explicitamente todos os zeros

use crate::{basic::{Matrix, MatrixInfo, Pair}};

/// Matriz densa implementada com vetor de vetores.
///
/// Armazena todos os valores da matriz em um `Vec<Vec<f64>>`, proporcionando
/// acesso O(1) a qualquer elemento, mas usando O(n²) de memória.
///
/// # Campos
///
/// * `size` - Dimensões da matriz como (linhas, colunas)
/// * `data` - Dados da matriz armazenados como vetor de vetores
///
/// # Exemplos
///
/// ```
/// use projeto::{TableMatrix, Matrix};
///
/// let mut matrix = TableMatrix::new((3, 3));
/// matrix.set((0, 0), 1.0);
/// matrix.set((1, 1), 2.0);
/// let value = matrix.get((0, 0)); // Retorna 1.0
/// ```
#[derive(Clone, Debug)]
pub struct TableMatrix {
	/// Dimensões da matriz como (linhas, colunas)
	pub size: Pair,
	/// Dados da matriz armazenados linha por linha
	pub data: Vec<Vec<f64>>,
}

impl TableMatrix {
	/// Cria uma nova matriz com as mesmas dimensões, inicializada com zeros.
	///
	/// # Retorno
	/// Nova matriz do mesmo tamanho com todos os valores zerados
	fn zero_like(&self) -> Self {
		TableMatrix::new(self.size)
	}
}

impl Matrix for TableMatrix {
	/// Cria uma nova matriz com as dimensões especificadas, inicializada com zeros.
	///
	/// # Complexidade
	/// - Tempo: O(n²) onde n é a dimensão
	/// - Espaço: O(n²)
	fn new(size: Pair) -> Self {
		TableMatrix {
			size,
			data: vec![vec![0.0; size.1]; size.0],
		}
	}
	
	/// Cria uma matriz a partir de metadados.
	///
	/// # Complexidade
	/// - Tempo: O(n² + k) onde k é o número de valores não-zero
	/// - Espaço: O(n²)
	fn from_info(info: &MatrixInfo) -> Self {
		let mut m = TableMatrix::new(info.size);
		for (pos, value) in info.values.iter() {
			let (r, c) = *pos;
			m.data[r][c] = *value;
		}
		m
	}

	/// Converte a matriz para metadados.
	///
	/// # Complexidade
	/// - Tempo: O(n²)
	/// - Espaço: O(n²) para armazenar todos os valores
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

	/// Retorna a transposta da matriz.
	///
	/// # Complexidade
	/// - Tempo: O(n²)
	/// - Espaço: O(n²) para a nova matriz
	fn transposed(self) -> Self {
		let mut t = TableMatrix::new((self.size.1, self.size.0));
		for i in 0..self.size.0 {
			for j in 0..self.size.1 {
				t.data[j][i] = self.data[i][j];
			}
		}
		t
	}
	
	/// Multiplica a matriz por um escalar.
	///
	/// # Complexidade
	/// - Tempo: O(n²)
	/// - Espaço: O(n²) para a nova matriz
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
	
	/// Multiplica duas matrizes.
	///
	/// Usa o algoritmo clássico de multiplicação de matrizes com otimização
	/// de localidade de cache (itera k no loop do meio).
	///
	/// # Complexidade
	/// - Tempo: O(n³)
	/// - Espaço: O(n²) para a matriz resultado
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
	
	/// Define o valor em uma posição específica.
	///
	/// # Complexidade
	/// - Tempo: O(1)
	fn set(&mut self, pos: Pair, value: f64) {
		self.data[pos.0][pos.1] = value;
	}
	
	/// Obtém o valor em uma posição específica.
	///
	/// # Complexidade
	/// - Tempo: O(1)
	fn get(&self, pos: Pair) -> f64 {
		self.data[pos.0][pos.1]
	}
	
	/// Adiciona duas matrizes elemento por elemento.
	///
	/// # Complexidade
	/// - Tempo: O(n²)
	/// - Espaço: O(n²) para a matriz resultado
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