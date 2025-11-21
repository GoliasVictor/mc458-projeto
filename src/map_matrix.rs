mod tree_map;
mod hash_map;
mod transposable_map;
pub use hash_map::HashMapStore;
pub use tree_map::TreeStore;
use transposable_map::TransposableMap;
use crate::basic::{Matrix, MatrixInfo, Pair};
use std::borrow::Cow; 


/// Estrutura que guarda um mapa de chaves de do K para valores do tipo U
/// A chave K e U deve ser clonavel.
pub trait Map<K : Copy, U : Clone > : Clone {
	// Cria um mapa a partir de um iterador de pares (K,U)
	fn from_iter<I: IntoIterator<Item=(K,U)>>(iter: I) -> Self;

	/// Insere ou atualiza o valor associado a chave
	fn set_or_insert(&mut self, key: K, value: U);
	
	/// Remove o valor associado a chave
	fn remove(&mut self, key: &K);
	
	/// Retorna uma referencia ao valor associado a chave, ou None se a chave nao existir
	fn get(&self, key: &K) -> Option<&U>;

	/// Retorna um iterador sobre os pares (K, U) do mapa
	/// Cow<'a, U> é copy-on-write, permitindo retornar referencias ou valores proprietarios dependendo do contexto, otimizando o uso de memoria
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(K, Cow<'a, U>)> + 'a>;


	/// Retorna um iterador mutavel sobre os pares (K, &mut U) do mapa
	/// Permite modificar os valores diretamente durante a iteraçao
	fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=(K, &'a mut U)> + 'a>;

}

/// Extensao do Map para valores que sao vetores, permitindo adicionar elementos ao vetor associado a chave
pub trait MapVec <K : Copy, U : Clone> : Map<K, Vec<U>> { 
	/// Adiciona um valor ao vetor associado a chave, criando o vetor se a chave nao existir
	fn add_to_vec(&mut self, key: K, value: U);
}


/// Matriz baseada em mapas para armazenar os valores
/// - `T`: tipo do mapa usado para armazenar os valores da matriz
/// - `LM`: tipo do mapa usado para armazenar os valores por linha ou coluna (usado na multiplicacao)
/// O tempo de cada uma das operações depende da implementaçao do mapa usado
/// Será represenado como T::operacao a complexidade de tempo da operaçao do mapa T
/// Será representando como T::full_iter a complexidade de tempo para iterar sobre todos os elementos do mapa T

pub struct MapMatrix <T:  Map<Pair, f64>, LM : MapVec<usize, (Pair, f64)>> {
	/// Dimensoes da matriz, representadas como um par (linhas, colunas)
    size: Pair,
	/// Mapa que armazena os valores da matriz, podendo ser transposto
    values: TransposableMap<T>,
	/// PhantomData para o tipo LM, usado na multiplicacao, serve para indicar que a struct depende do tipo LM sem armazenar um valor dele
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