//! Biblioteca para implementação e comparação de diferentes estruturas de matrizes esparsas.
//!
//! Esta biblioteca fornece implementações eficientes de matrizes esparsas e densas,
//! permitindo comparar o desempenho de diferentes estruturas de dados para operações
//! matriciais.
//!
//! # Implementações Disponíveis
//!
//! - **`TableMatrix`**: Representação densa usando `Vec<Vec<f64>>`
//! - **`HashMapMatrix`**: Representação esparsa usando `HashMap`
//! - **`TreeMatrix`**: Representação esparsa usando `BTreeMap`
//!
//! # Exemplos
//!
//! ```
//! use projeto::{HashMapMatrix, Matrix};
//!
//! // Criar uma matriz 3x3
//! let mut matrix = HashMapMatrix::new((3, 3));
//!
//! // Definir valores
//! matrix.set((0, 0), 1.0);
//! matrix.set((1, 1), 2.0);
//! matrix.set((2, 2), 3.0);
//!
//! // Obter um valor
//! let value = matrix.get((0, 0)); // Retorna 1.0
//!
//! // Transpor a matriz
//! let transposed = matrix.transposed();
//! ```
//!
//! # Operações Suportadas
//!
//! Todas as implementações suportam:
//! - Criação de matrizes
//! - Acesso e modificação de elementos
//! - Transposição
//! - Adição de matrizes
//! - Multiplicação de matrizes
//! - Multiplicação por escalar

#![allow(dead_code)]
mod map_matrix;
mod table_matrix;
mod basic;
pub mod alloc;
use std::{collections::{HashMap}};
pub use crate::{basic::{Matrix, MatrixInfo, Pair}, map_matrix::{HashMapStore, MapMatrix, TreeStore}};

// Type aliases para facilitar o uso das diferentes implementações de matrizes

/// Matriz esparsa baseada em HashMap.
///
/// Esta implementação usa `HashMap` para armazenar apenas valores não-zero,
/// proporcionando acesso médio O(1) e eficiência de memória para matrizes esparsas.
///
/// # Características
/// - Acesso: O(1) médio
/// - Memória: O(k) onde k é o número de elementos não-zero
/// - Bom para: Matrizes muito esparsas com padrão de acesso aleatório
pub type HashMapMatrix = MapMatrix<HashMapStore<Pair, f64>, HashMapStore<usize, Vec<(Pair, f64)>>>;

/// Matriz esparsa baseada em BTreeMap.
///
/// Esta implementação usa `BTreeMap` para armazenar valores não-zero de forma ordenada,
/// proporcionando melhor localidade de cache e iteração ordenada.
///
/// # Características
/// - Acesso: O(log k) onde k é o número de elementos não-zero
/// - Memória: O(k) onde k é o número de elementos não-zero
/// - Bom para: Matrizes esparsas onde a ordem é importante
pub type TreeMatrix = MapMatrix<TreeStore<Pair, f64>, TreeStore<usize, Vec<(Pair, f64)>>>;

/// Matriz densa baseada em tabela (vetor de vetores).
///
/// Esta implementação usa `Vec<Vec<f64>>` para armazenar todos os elementos da matriz,
/// incluindo zeros.
///
/// # Características
/// - Acesso: O(1) direto
/// - Memória: O(n²) onde n é a dimensão da matriz
/// - Bom para: Matrizes pequenas ou densas
pub type TableMatrix = table_matrix::TableMatrix;

/// Epsilon para comparações de ponto flutuante.
///
/// Usado para lidar com imprecisões de aritmética de ponto flutuante
/// ao comparar valores f64.
pub const EPSILON : f64 = 1e-8;


/// Compara duas matrizes representadas como `MatrixInfo` para igualdade.
///
/// Usa `EPSILON` para comparação de ponto flutuante, considerando valores
/// iguais se diferem por menos que `EPSILON`.
///
/// # Argumentos
/// * `expected` - MatrixInfo esperado
/// * `current` - MatrixInfo atual
///
/// # Retorno
/// `true` se as matrizes são equivalentes, `false` caso contrário
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

/// Calcula as diferenças entre duas matrizes representadas como `MatrixInfo`.
///
/// # Argumentos
/// * `expected` - MatrixInfo esperado
/// * `current` - MatrixInfo atual
///
/// # Retorno
/// Vetor de tuplas contendo (posição, (valor_esperado, valor_atual)) para cada
/// posição onde os valores diferem
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

/// Multiplica duas matrizes usando uma implementação específica de Matrix.
///
/// # Argumentos de Tipo
/// * `M` - Tipo da implementação de Matrix a usar
///
/// # Argumentos
/// * `ainfo` - Primeira matriz como MatrixInfo
/// * `binfo` - Segunda matriz como MatrixInfo
///
/// # Retorno
/// MatrixInfo representando o produto das duas matrizes
fn mul<M :  Matrix>(ainfo: &MatrixInfo, binfo: &MatrixInfo)  -> MatrixInfo {
    let a = M::from_info(ainfo).transposed();
    let b = M::from_info(binfo).transposed();
    M::mul(&b, &a).to_info()
    

}
