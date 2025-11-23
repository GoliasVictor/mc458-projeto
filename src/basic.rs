//! Definições básicas e trait Matrix para operações com matrizes.
//!
//! Este módulo define o trait fundamental `Matrix` que especifica as operações
//! básicas que todas as implementações de matrizes devem suportar.

/// Tipo para representar uma posição ou dimensão de matriz como (linha, coluna).
///
/// # Exemplos
/// ```
/// let position: Pair = (2, 3); // linha 2, coluna 3
/// let dimensions: Pair = (10, 10); // matriz 10x10
/// ```
pub type Pair = (usize, usize); 

/// Trait que define as operações básicas para matrizes.
///
/// Este trait fornece uma interface comum para diferentes implementações de matrizes,
/// permitindo que sejam usadas de forma intercambiável. Todas as implementações devem
/// suportar operações básicas de álgebra linear.
///
/// # Implementações
/// - `TableMatrix`: Implementação densa usando vetores
/// - `MapMatrix<HashMapStore, _>`: Implementação esparsa usando HashMap
/// - `MapMatrix<TreeStore, _>`: Implementação esparsa usando BTreeMap
pub trait Matrix {
    /// Cria uma nova matriz com as dimensões especificadas.
    ///
    /// # Argumentos
    /// * `size` - Dimensões da matriz como (linhas, colunas)
    ///
    /// # Retorno
    /// Uma nova matriz inicializada (geralmente com zeros)
    fn new(size: Pair) -> Self;
    
    /// Define o valor em uma posição específica da matriz.
    ///
    /// # Argumentos
    /// * `pos` - Posição como (linha, coluna)
    /// * `value` - Valor a ser definido
    fn set(&mut self, pos: Pair, value: f64);
    
    /// Obtém o valor em uma posição específica da matriz.
    ///
    /// # Argumentos
    /// * `pos` - Posição como (linha, coluna)
    ///
    /// # Retorno
    /// O valor na posição especificada (0.0 se não definido)
    fn get(&self, pos: Pair) -> f64;
    
    /// Retorna a transposta da matriz.
    ///
    /// A transposta de uma matriz A é uma matriz A^T onde A^T\[i,j\] = A\[j,i\].
    ///
    /// # Retorno
    /// Uma nova matriz que é a transposta da matriz atual
    fn transposed(self) -> Self;
    
    /// Adiciona duas matrizes.
    ///
    /// # Argumentos
    /// * `a` - Primeira matriz
    /// * `b` - Segunda matriz
    ///
    /// # Retorno
    /// Uma nova matriz que é a soma de a e b (C\[i,j\] = A\[i,j\] + B\[i,j\])
    fn add(a : &Self, b : &Self) -> Self;
    
    /// Multiplica duas matrizes.
    ///
    /// # Argumentos
    /// * `a` - Primeira matriz (dimensões m×n)
    /// * `b` - Segunda matriz (dimensões n×p)
    ///
    /// # Retorno
    /// Uma nova matriz resultante da multiplicação (dimensões m×p)
    fn mul(a : &Self, b : &Self) -> Self;
    
    /// Multiplica uma matriz por um escalar.
    ///
    /// # Argumentos
    /// * `a` - Matriz a ser multiplicada
    /// * `scalar` - Valor escalar
    ///
    /// # Retorno
    /// Uma nova matriz onde cada elemento é multiplicado pelo escalar
    fn muls(a : &Self, scalar: f64) -> Self;
    
    /// Cria uma matriz a partir de metadados.
    ///
    /// # Argumentos
    /// * `info` - Metadados da matriz (dimensões e valores)
    ///
    /// # Retorno
    /// Uma nova matriz criada a partir dos metadados
	fn from_info(info: &MatrixInfo) -> Self;
	
	/// Converte a matriz para metadados.
    ///
    /// # Retorno
    /// Metadados contendo dimensões e valores da matriz
	fn to_info(&self) -> MatrixInfo;
}

/// Metadados de uma matriz: suas dimensões e entradas armazenadas.
///
/// `MatrixInfo` coleta as informações essenciais necessárias para descrever uma matriz:
/// - `size`: um `Pair` descrevendo as dimensões da matriz (linhas, colunas).
/// - `values`: um `Vec<(Pair, f64)>` contendo as entradas como `(posição, valor)`,
///   onde `posição` é um `Pair` (linha, coluna).
///
/// # Uso
/// Esta estrutura é útil para serialização, deserialização e conversão entre
/// diferentes implementações de matrizes.
///
/// # Exemplos
/// ```
/// use projeto::{MatrixInfo, Pair};
///
/// let info = MatrixInfo {
///     size: (3, 3),
///     values: vec![
///         ((0, 0), 1.0),
///         ((1, 1), 2.0),
///         ((2, 2), 3.0),
///     ],
/// };
/// ```
#[derive(Clone)]
pub struct MatrixInfo {
    /// Dimensões da matriz como (linhas, colunas)
    pub size: Pair,
    /// Vetor de entradas não-zero como (posição, valor)
    pub values: Vec<(Pair, f64)>
}

impl MatrixInfo {
    /// Imprime todos os valores da matriz.
    ///
    /// Útil para debugging e visualização de matrizes pequenas.
    ///
    /// # Exemplos
    /// ```
    /// use projeto::MatrixInfo;
    ///
    /// let info = MatrixInfo {
    ///     size: (2, 2),
    ///     values: vec![((0, 0), 1.0), ((1, 1), 2.0)],
    /// };
    /// info.print_values();
    /// // Saída:
    /// // (0, 0) = 1
    /// // (1, 1) = 2
    /// ```
	pub fn print_values(&self) {
		for (pos, value) in self.values.iter() {
			println!("{:?} = {}", pos, value);
		}
	}
}