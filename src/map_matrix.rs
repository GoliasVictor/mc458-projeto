//! Implementação de matrizes esparsas baseadas em mapas.
//!
//! Este módulo fornece `MapMatrix`, uma implementação genérica de matriz esparsa
//! que pode usar diferentes tipos de mapas (HashMap, BTreeMap, etc.) para armazenar
//! apenas valores não-zero.
//!
//! # Características Principais
//!
//! - **Eficiência de Memória**: Armazena apenas valores não-zero
//! - **Transposição O(1)**: Usa flag de transposição ao invés de realocar
//! - **Genérica**: Funciona com qualquer implementação de Map
//!
//! # Tipos Disponíveis
//!
//! - `HashMapStore`: Implementação usando `HashMap` (acesso O(1) médio)
//! - `TreeStore`: Implementação usando `BTreeMap` (acesso O(log k))

mod tree_map;
mod hash_map;
mod transposable_map;
pub use hash_map::HashMapStore;
pub use tree_map::TreeStore;
use transposable_map::TransposableMap;
use crate::basic::{Matrix, MatrixInfo, Pair};
use std::borrow::Cow; 


/// Trait para estruturas de mapa que podem ser usadas em `MapMatrix`.
///
/// Define as operações básicas necessárias para implementar uma matriz esparsa.
/// A chave `K` e o valor `U` devem ser clonáveis.
///
/// # Parâmetros de Tipo
/// - `K`: Tipo da chave (deve implementar `Copy`)
/// - `U`: Tipo do valor (deve implementar `Clone`)
///
/// # Implementações
/// - `HashMapStore`: Baseado em `HashMap`
/// - `TreeStore`: Baseado em `BTreeMap`
pub trait Map<K : Copy, U : Clone > : Clone {
	/// Cria um mapa a partir de um iterador de pares (chave, valor).
	///
	/// # Argumentos
	/// * `iter` - Iterador de tuplas (K, U)
	fn from_iter<I: IntoIterator<Item=(K,U)>>(iter: I) -> Self;

	/// Insere ou atualiza o valor associado à chave.
	///
	/// Se a chave já existe, seu valor é atualizado.
	///
	/// # Argumentos
	/// * `key` - Chave a ser inserida/atualizada
	/// * `value` - Novo valor
	fn set_or_insert(&mut self, key: K, value: U);
	
	/// Remove o valor associado à chave.
	///
	/// # Argumentos
	/// * `key` - Chave a ser removida
	fn remove(&mut self, key: &K);
	
	/// Retorna uma referência ao valor associado à chave.
	///
	/// # Argumentos
	/// * `key` - Chave a ser consultada
	///
	/// # Retorno
	/// `Some(&U)` se a chave existe, `None` caso contrário
	fn get(&self, key: &K) -> Option<&U>;

	/// Retorna um iterador sobre os pares (chave, valor) do mapa.
	///
	/// Usa `Cow` (copy-on-write) para permitir retornar referências ou valores
	/// proprietários dependendo do contexto, otimizando o uso de memória.
	///
	/// # Retorno
	/// Iterador sobre pares (K, Cow<'a, U>)
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(K, Cow<'a, U>)> + 'a>;

	/// Retorna um iterador mutável sobre os pares (chave, valor) do mapa.
	///
	/// Permite modificar os valores diretamente durante a iteração.
	///
	/// # Retorno
	/// Iterador sobre pares (K, &'a mut U)
	fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=(K, &'a mut U)> + 'a>;

}

/// Extensão do trait `Map` para mapas cujos valores são vetores.
///
/// Fornece uma operação conveniente para adicionar elementos aos vetores
/// associados a chaves.
///
/// # Parâmetros de Tipo
/// - `K`: Tipo da chave (deve implementar `Copy`)
/// - `U`: Tipo dos elementos do vetor (deve implementar `Clone`)
pub trait MapVec <K : Copy, U : Clone> : Map<K, Vec<U>> { 
	/// Adiciona um valor ao vetor associado à chave.
	///
	/// Se a chave não existe, cria um novo vetor contendo o valor.
	///
	/// # Argumentos
	/// * `key` - Chave do vetor
	/// * `value` - Valor a ser adicionado ao vetor
	fn add_to_vec(&mut self, key: K, value: U);
}


/// Matriz esparsa genérica baseada em mapas.
///
/// Esta estrutura implementa uma matriz esparsa que armazena apenas valores não-zero
/// usando um mapa genérico. A escolha do tipo de mapa afeta o desempenho das operações.
///
/// # Parâmetros de Tipo
///
/// - `T`: Tipo do mapa usado para armazenar os valores da matriz
/// - `LM`: Tipo do mapa usado para armazenar valores por linha/coluna (usado na multiplicação)
///
/// # Complexidade das Operações
///
/// O tempo de cada operação depende da implementação do mapa usado:
/// - T::operacao: complexidade da operação no mapa T
/// - T::full_iter: complexidade para iterar sobre todos os elementos do mapa T
///
/// # Exemplos
///
/// ```
/// use projeto::{HashMapMatrix, Matrix};
///
/// let mut matrix = HashMapMatrix::new((100, 100));
/// matrix.set((0, 0), 1.0);
/// matrix.set((50, 50), 2.0);
/// // Usa apenas memória para 2 elementos, não 10000
/// ```
pub struct MapMatrix <T:  Map<Pair, f64>, LM : MapVec<usize, (Pair, f64)>> {
	/// Dimensões da matriz, representadas como um par (linhas, colunas)
    size: Pair,
	/// Mapa que armazena os valores não-zero da matriz, podendo ser transposto
    values: TransposableMap<T>,
	/// PhantomData para o tipo LM. Indica que a struct depende do tipo LM
	/// sem armazenar um valor dele. Usado na multiplicação de matrizes.
	phatom: std::marker::PhantomData<LM>
}

impl<T:  Map<Pair, f64>, LM : MapVec<usize, (Pair, f64)>> Matrix for MapMatrix<T, LM> {
	/// Cria uma nova matriz com as dimensoes especificadas, inicialmente vazia
	/// Complexidade de tempo: O(1)
	/// Complexidade de espaco: O(1)
	fn new(size: Pair) -> MapMatrix<T, LM>{
		MapMatrix {
			size,
			values: TransposableMap::new(T::from_iter(std::iter::empty())),
			phatom: std::marker::PhantomData
		}
	}
	/// Retorna uma nova matriz que é o produto da matriz atual com um escalar
	/// Complexidade de tempo: O(n * T::set_or_insert(n)), onde n é o numero de elementos na matriz
	fn muls(a : &Self, scalar: f64) -> Self {
		let mut c = a.values.clone();
		for (_, mut value) in c.iter_mut(){
			*value = *value * scalar;
		}
		return MapMatrix {
			size: a.size,
			values: c,
			phatom: std::marker::PhantomData
		};
	}
	/// Define o valor na posiçao especificada
	/// Complexidade de tempo: O(T::set_or_insert(n)  + T::remove(n)), onde n é o numero de elementos no mapa
	fn set(&mut self, pos: Pair, value: f64) {
        if value == 0.0 {
            self.values.remove(&pos);
        } else {
            self.values.set_or_insert(pos, value);
        }
    }
	/// Retorna o valor na posiçao especificada, retornando 0.0 se nao houver valor definido
	/// Complexidade de tempo: O(T::get(n)), onde n é o numero de elementos no mapa
    fn get(&self, pos: Pair) -> f64 {
        *self.values.get(&pos).unwrap_or(&0.0)
    }
	/// Retorna uma nova matriz que é a transposta da matriz atual
	/// Complexidade de tempo: O(1)
    fn transposed(mut self) -> MapMatrix<T, LM> {
		self.size = (self.size.1, self.size.0);
        self.values.transpose();
		self
    }

	/// Retorna uma nova matriz que é a soma da matriz atual com outra matriz
	/// Complexidade de tempo: O( (ka + kb) * (T::set_or_insert(kc) + T::get(kc))),
	/// Onde ka é o numero de elementos na matriz a, kb é o numero de elementos na matriz b, e kc é o numero de elementos na matriz resultante
    fn add(a : &MapMatrix<T, LM>, b : &MapMatrix<T, LM>) -> MapMatrix<T, LM> {
        let mut c = MapMatrix { 
			size: a.size,
			values: a.values.clone(),
			phatom: std::marker::PhantomData
		};
		for (pos, vb) in b.values.iter()  {
			let value =  a.get(pos)+ *vb;
			c.set(pos, value);
		}	
        return c
    }
	/// Retorna uma nova matriz que é o produto da matriz atual com outra matriz
	/// 
	/// Estrutura:
	/// - Separação: Primeiro a função cria dois mapas auxiliares, um para armazenar os valores de cada coluna da matriz a.
	/// - Mutiplicação: Então a função itera sobre as colunas da matriz a e linhas da matriz b, multiplicando os valores correspondentes e somando-os na matriz resultante.
	/// Complexidade de tempo: O(ka * kb / n * (T::get(kc) + T::set_or_insert(kc))),
    fn mul(a : &MapMatrix<T, LM>, b : &MapMatrix<T, LM>) -> MapMatrix<T, LM> {
        let mut c = MapMatrix::new((a.size.0, b.size.1));
		let mut acolumns = LM::from_iter(std::iter::empty()); 
		let mut brows = LM::from_iter(std::iter::empty());
		for (apos, va) in a.values.iter()  {
			acolumns.add_to_vec(apos.1, (apos, *va));
		}
		for (bpos, vb) in b.values.iter() {
			brows.add_to_vec(bpos.0, (bpos, *vb));
		}
		for (i, avalues) in acolumns.iter() {
			let Some(bvalues) = brows.get(&i) else {
				continue;
			};
			for (apos, va) in avalues.iter()  {
				for (bpos, vb) in  bvalues.iter() {
					assert_eq!(a.size.1, b.size.0, "Incompatible matrices for multiplication");
					let pos = (apos.0, bpos.1);
					let value =  c.get(pos)+ vb*va;
					c.set(pos, value);
				}
			}
		}
        return c;
    }

	/// Converte a matriz para uma estrutura MatrixInfo, que armazena as dimensoes e os valores da matriz
	/// Complexidade de tempo: O(T::full_iter(n)), onde n é o numero de elementos na matriz
	fn to_info(&self) -> MatrixInfo {
		let mut values = Vec::new();
		for (pos, value) in self.values.iter() {
			values.push(( pos, value.into_owned()));
		}
		MatrixInfo {
			size: self.size,
			values
		}
	}
	/// Cria uma matriz a partir de uma estrutura MatrixInfo
	/// Complexidade de tempo: O(n * T::set_or_insert(n)), onde n é o numero de elementos na MatrixInfo
	fn from_info(info: &MatrixInfo) -> Self {
		MapMatrix {
			size: info.size,
			values: TransposableMap::new(T::from_iter(info.values.iter().map(|(pos, value)| (*pos, *value)))),
			phatom: std::marker::PhantomData
		}
	}
}