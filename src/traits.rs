//! Traits for elements of AVL tree.

use num_traits::Zero;
use rand::distributions::{Uniform, Distribution};

use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;

/// Elements of AVL tree.
pub trait TreeElem: PartialEq + PartialOrd + Copy + fmt::Display + Zero {}

impl TreeElem for f32 {}
impl TreeElem for f64 {}
impl TreeElem for i8 {}
impl TreeElem for i16 {}
impl TreeElem for i32 {}
impl TreeElem for i64 {}
impl TreeElem for i128 {}
impl TreeElem for isize {}
impl TreeElem for u8 {}
impl TreeElem for u16 {}
impl TreeElem for u32 {}
impl TreeElem for u64 {}
impl TreeElem for u128 {}
impl TreeElem for usize {}
impl<T: TreeElem> TreeElem for OrdEqElem<T> {}

/// Integer and float with `Ord` and `Eq` trait.
/// 
/// When comparing two values, if they are equal,
/// the order is determined randomly.
#[derive(Clone, Copy)]
pub struct OrdEqElem<T: TreeElem> {
    pub value: T,
}

impl<T: TreeElem> OrdEqElem<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: TreeElem> Ord for OrdEqElem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.partial_cmp(&other.value) {
            Some(order) => match order {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                    let ud = Uniform::new(0.0f32, 1.0f32);
                    if ud.sample(&mut rand::thread_rng()) < 0.5 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
            None => panic!(),
        }
    }
}

impl<T: TreeElem> PartialOrd for OrdEqElem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: TreeElem> PartialEq for OrdEqElem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: TreeElem> Eq for OrdEqElem<T> {}

impl<T: TreeElem> std::fmt::Display for OrdEqElem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: TreeElem> Add for OrdEqElem<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self { value: self.value + other.value }
    }
}

impl<T: TreeElem> Zero for OrdEqElem<T> {
    fn zero() -> Self {
        Self { value: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.value == T::zero()
    }
}