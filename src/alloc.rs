//! Módulo de rastreamento de alocações de memória.
//!
//! Este módulo fornece um alocador personalizado (`TrackingAllocator`) que
//! monitora todas as alocações e desalocações de memória durante a execução
//! do programa. É usado principalmente para medir com precisão o uso de memória
//! das diferentes implementações de matriz nos benchmarks.
//!
//! # Uso
//!
//! ```
//! use projeto::alloc::{reset, stats};
//!
//! // Reinicia os contadores
//! reset();
//!
//! // ... executa código que aloca memória ...
//!
//! // Obtém estatísticas
//! let stats = stats();
//! println!("Alocado: {} bytes", stats.alloc);
//! println!("Desalocado: {} bytes", stats.dealloc);
//! println!("Diferença: {} bytes", stats.diff);
//! ```
//!
//! # Referência
//! Baseado em: https://www.ntietz.com/blog/rust-hashmap-overhead/

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Contador global de bytes alocados
static ALLOC: AtomicUsize = AtomicUsize::new(0);
/// Contador global de bytes desalocados
static DEALLOC: AtomicUsize = AtomicUsize::new(0);

/// Alocador personalizado que rastreia alocações e desalocações de memória.
///
/// Este alocador envolve o alocador do sistema e registra o tamanho de cada
/// alocação e desalocação em contadores atômicos globais.
pub struct TrackingAllocator;

/// Registra uma alocação de memória.
///
/// # Argumentos
/// * `layout` - Layout da alocação contendo tamanho e alinhamento
pub fn record_alloc(layout: Layout) {
    ALLOC.fetch_add(layout.size(), Ordering::SeqCst);
}

/// Registra uma desalocação de memória.
///
/// # Argumentos
/// * `layout` - Layout da desalocação contendo tamanho e alinhamento
pub fn record_dealloc(layout: Layout) {
    DEALLOC.fetch_add(layout.size(), Ordering::SeqCst);
}

/// Reinicia os contadores de alocação e desalocação para zero.
///
/// Útil para isolar medições de diferentes testes ou benchmarks.
pub fn reset() {
    ALLOC.store(0, Ordering::SeqCst);
    DEALLOC.store(0, Ordering::SeqCst);
}

/// Obtém as estatísticas atuais de alocação de memória.
///
/// # Retorno
/// Uma estrutura `Stats` contendo:
/// - `alloc`: Total de bytes alocados
/// - `dealloc`: Total de bytes desalocados
/// - `diff`: Diferença (bytes atualmente em uso)
pub fn stats() -> Stats {
    let alloc = ALLOC.load(Ordering::SeqCst);
    let dealloc = DEALLOC.load(Ordering::SeqCst);
    let diff = (alloc as isize) - (dealloc as isize);

    Stats {
        alloc,
        dealloc,
        diff,
    }
}

/// Estatísticas de uso de memória.
///
/// # Campos
/// * `alloc` - Total de bytes alocados
/// * `dealloc` - Total de bytes desalocados
/// * `diff` - Diferença entre alocado e desalocado (uso atual)
pub struct Stats {
    /// Total de bytes alocados desde o último reset
    pub alloc: usize,
    /// Total de bytes desalocados desde o último reset
    pub dealloc: usize,
    /// Diferença entre alocado e desalocado (pode ser negativo)
    pub diff: isize, 
}

unsafe impl GlobalAlloc for TrackingAllocator {
    /// Aloca memória e registra o tamanho da alocação.
    ///
    /// # Safety
    /// Delega ao alocador do sistema após registrar a alocação.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe {
			let p = System.alloc(layout);
			record_alloc(layout);
			p
		}
    }

    /// Desaloca memória e registra o tamanho da desalocação.
    ///
    /// # Safety
    /// Registra a desalocação antes de delegar ao alocador do sistema.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe {
			record_dealloc(layout); 
			System.dealloc(ptr, layout);
		}
    }
}

/// Alocador global usado pelo programa.
///
/// Todas as alocações de memória no programa usarão este alocador,
/// permitindo rastreamento completo do uso de memória.
#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;