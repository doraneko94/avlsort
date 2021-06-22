//! Rust implementation of AVL tree.
//! 
//! This tree holds the rank and the number of duplicates of the elements,
//! and returns them when a new element is added.
//! 
//! ## Usage
//! 
//! examples/avl.rs
//! 
//! ```rust
//! use avlsort::tree::*;
//!
//! fn main() {
//!    let v = vec![1, 8, 7, 3, 5, 6, 2, 9, 4, 0, 4, 9];
//!    let mut g = AvlTree::new();
//!    println!("=== Push values in v. ===");
//!    for &i in v.iter() {
//!        let (rank, dup) = g.push(i);
//!        println!("value={}| rank={}, dup={}", i, rank, dup);
//!    }
//!    println!("Elements in the tree = {}", g.len());
//!    println!("");
//!
//!    println!("Height of the tree = {}", g.height());
//!    println!("How many 4?: {}", g.count(4));
//!    println!("remove 4: {:?}", g.remove(4));
//!    println!("4 is in?: {}", g.isin(4));
//!    println!("remove 4: {:?}", g.remove(4));
//!    println!("4 is in?: {}", g.isin(4));
//!    println!("");
//!
//!    println!("Max value = {}", g.max().unwrap());
//!    let (value, dup) = g.pop_max_all().unwrap();
//!    println!("Pop all the max value: value={}, dup={}", value, dup);
//!    println!("");
//!
//!    println!("=== Show elements in ascending order. ===");
//!    loop {
//!        match g.pop_min() {
//!            Some(value) => println!("Min value = {}", value),
//!            None => break,
//!        }
//!    }
//! }
//! ```

pub mod node;
pub mod traits;
pub mod tree;