//! The node of AVL tree.

use std::sync::{Arc, Mutex};

use crate::traits::*;

/// When propagating tree height information, 
/// the direction of the child from which the information cames is indicated.
pub enum Direction {
    Left,
    Right,
}

/// Information on changing the height of tree.
#[derive(Clone, Copy)]
pub enum DeltaDiff {
    Longer,
    Zero,
    Shorter,
}

/// The node of AVL tree.
pub struct AvlNode<T> {
    /// The value of element.
    pub value: T,
    /// The difference of heights of children.
    /// 
    /// If diff > 0, the height of the left child is larger than that of the right child.
    pub diff: i32,
    /// The number of elements smaller than the value of this node, 
    /// and the number of duplicates of `value`.
    /// 
    /// `(number_of_less, number_of_duplicates)`
    pub n_ledu: (usize, usize),
    /// Pointer to the left child node.
    pub left: Option<Arc<Mutex<Self>>>,
    /// Pointer to the right child node.
    pub right: Option<Arc<Mutex<Self>>>,
}

impl<T: TreeElem> AvlNode<T> {
    /// Create a new node.
    pub fn new(value: T) -> Self {
        Self { value, diff: 0, n_ledu: (0, 0), left: None, right: None }
    }

    /// Push `value` and propagate `((number_of_less, number_of_duplicates), height_information)` to parent node.
    pub fn push_child(&mut self, value: T) -> ((usize, usize), DeltaDiff) {
        if value < self.value {
            self.n_ledu.0 += 1;
            match &self.left {
                Some(node) => {
                    let (n_ledu, d_diff) = node.lock().unwrap().push_child(value);
                    
                    (n_ledu, self.balance(d_diff, Direction::Left))
                }
                _ => {
                    self.left = Some(Arc::new(Mutex::new(Self::new(value))));
                    
                    ((0, 0), self.balance(DeltaDiff::Longer, Direction::Left))
                }
            }
        } else if value > self.value {
            match &self.right {
                Some(node) => {
                    let (n_ledu, d_diff) = node.lock().unwrap().push_child(value);
                    
                    ((n_ledu.0 + self.n_ledu.0 + self.n_ledu.1 + 1, n_ledu.1), self.balance(d_diff, Direction::Right))
                }
                _ => {
                    self.right = Some(Arc::new(Mutex::new(Self::new(value))));
                    
                    ((self.n_ledu.0 + self.n_ledu.1 + 1, 0), self.balance(DeltaDiff::Longer, Direction::Right))
                }
            }
        } else {
            self.n_ledu.1 += 1;
            (self.n_ledu, DeltaDiff::Zero)
        }
    }

    /// Search `value` and propagate `number_of_duplicates` to parent node.
    pub fn search(&self, value: T) -> Option<usize> {
        if value == self.value {
            Some(self.n_ledu.1)
        } else {
            if value < self.value {
                match &self.left {
                    Some(node) => node.lock().unwrap().search(value),
                    None => None,
                }
            } else {
                match &self.right {
                    Some(node) => node.lock().unwrap().search(value),
                    None => None,
                }
            }
        }
    }

    /// Remove the node of `value` and propagate `height_information` to parent node.
    /// 
    /// If `value` is a duplicate, return and remove only one.
    pub fn remove_child(&mut self, value: T) -> Result<DeltaDiff, ()> {
        if value == self.value {
            return Err(());
        }
        if value < self.value {
            let (new_child, fin, reconnect) = match &self.left {
                Some(node) => {
                    let mut n = node.lock().unwrap();
                    if n.value == value {
                        if n.n_ledu.1 > 0 {
                            n.n_ledu.1 -= 1;
                            (None, Ok(Some(DeltaDiff::Zero)), false)
                        } else {
                            n.remove_reconnect()
                        }
                    } else {
                        (None, Err(()), false)
                    }
                }
                None => (None, Ok(None), false),
            };
            if reconnect {
                self.left = new_child;
            }
            match fin {
                Ok(Some(d_diff)) => {
                    Ok(self.balance(d_diff, Direction::Left))
                }
                Ok(None) => {
                    Err(())
                }
                Err(()) => {
                    match self.left.clone().unwrap().lock().unwrap().remove_child(value) {
                        Ok(d_diff) => {
                            Ok(self.balance(d_diff, Direction::Left))
                        }
                        Err(()) => Err(())
                    }
                }
            }
        } else {
            let (new_child, fin, reconnect) = match &self.right {
                Some(node) => {
                    let mut n = node.lock().unwrap();
                    if n.value == value {
                        if n.n_ledu.1 > 0 {
                            n.n_ledu.1 -= 1;
                            (None, Ok(Some(DeltaDiff::Zero)), false)
                        } else {
                            n.remove_reconnect()
                        }
                    } else {
                        (None, Err(()), false)
                    }
                }
                None => (None, Ok(None), false),
            };
            if reconnect {
                self.right = new_child;
            }
            match fin {
                Ok(Some(d_diff)) => {
                    Ok(self.balance(d_diff, Direction::Right))
                }
                Ok(None) => {
                    Err(())
                }
                Err(()) => {
                    match self.right.clone().unwrap().lock().unwrap().remove_child(value) {
                        Ok(d_diff) => {
                            Ok(self.balance(d_diff, Direction::Right))
                        }
                        Err(()) => Err(())
                    }
                }
            }
        }
    }

    /// Utility function for removing a node.
    fn remove_reconnect(&mut self) -> (Option<Arc<Mutex<Self>>>, Result<Option<DeltaDiff>, ()>, bool) {
        match (&self.left, &self.right) {
            (Some(nl), Some(nr)) => {
                let n_m = if self.diff >= 0 {
                    nl.lock().unwrap().max_child()
                } else {
                    nr.lock().unwrap().min_child()
                };
                self.value = n_m;
                self.n_ledu.1 = self.search(n_m).unwrap();
                match self.remove_child(n_m) {
                    Ok(d_diff) => (None, Ok(Some(d_diff)), false),
                    Err(()) => panic!(),
                }
            }
            (Some(nl), None) => {
                (Some(nl.clone()), Ok(Some(self.balance(DeltaDiff::Shorter, Direction::Left))), true)
            }
            (None, Some(nr)) => {
                (Some(nr.clone()), Ok(Some(self.balance(DeltaDiff::Shorter, Direction::Right))), true)
            }
            (None, None) => {
                (None, Ok(Some(DeltaDiff::Shorter)), true)
            }
        }
    }

    /// Balance the tree at the bottom and propagate `height_information` to parent node.
    pub fn balance(&mut self, d_diff: DeltaDiff, from_dir: Direction) -> DeltaDiff {
        match d_diff {
            DeltaDiff::Zero => DeltaDiff::Zero,
            DeltaDiff::Longer => {
                match from_dir {
                    Direction::Left => {
                        self.diff += 1;
                        if self.rotate() {
                            DeltaDiff::Zero
                        } else {
                            if self.diff > 0 {
                                DeltaDiff::Longer
                            } else {
                                DeltaDiff::Zero
                            }
                        }
                    }
                    Direction::Right => {
                        self.diff -= 1;
                        if self.rotate() {
                            DeltaDiff::Zero
                        } else {
                            if self.diff < 0 {
                                DeltaDiff::Longer
                            } else {
                                DeltaDiff::Zero
                            }
                        }
                    }
                }
            }
            DeltaDiff::Shorter => {
                match from_dir {
                    Direction::Left => {
                        self.diff -= 1;
                        if self.diff == -2 {
                            let d_diff_rotate = match &self.right {
                                None => panic!(),
                                Some(node) => {
                                    if node.lock().unwrap().diff == 0 {
                                        DeltaDiff::Zero
                                    } else {
                                        DeltaDiff::Shorter
                                    }
                                }
                            };
                            let _ = self.rotate();
                            d_diff_rotate
                        } else if self.diff <= 1 && self.diff >= -1 {
                            if self.diff >= 0 {
                                DeltaDiff::Shorter
                            } else {
                                DeltaDiff::Zero
                            }
                        } else {
                            panic!()
                        }
                    }
                    Direction::Right => {
                        self.diff += 1;
                        if self.diff == 2 {
                            let d_diff_rotate = match &self.left {
                                None => panic!(),
                                Some(node) => {
                                    if node.lock().unwrap().diff == 0 {
                                        DeltaDiff::Zero
                                    } else {
                                        DeltaDiff::Shorter
                                    }
                                }
                            };
                            d_diff_rotate
                        } else if self.diff <= 1 && self.diff >= -1 {
                            if self.diff <= 0 {
                                DeltaDiff::Shorter
                            } else {
                                DeltaDiff::Zero
                            }
                        } else {
                            panic!()
                        }
                    }
                }
            }
        }
    }

    /// Rotate the tree at the bottom to balance it.
    pub fn rotate(&mut self) -> bool {
        if self.diff <= 1 && self.diff >= -1 {
            return false;
        }
        if self.diff == 2 {
            let n_val = self.value;
            let n_n_ledu1 = self.n_ledu.1;
            let nr = self.right.clone();
            let (nl_val, nl_n_ledu1, n_diff, nll_op) = match &mut self.left {
                None => panic!(),
                Some(nl_arc) => {
                    let mut nl = nl_arc.lock().unwrap();
                    let nl_val = nl.value;
                    let nl_n_ledu1 = nl.n_ledu.1;
                    let nll_op = nl.left.clone();
                    if nl.diff == -1 {
                        let (nlr_val, nlr_n_ledu1, nlrr_op, diff) = match &mut nl.right {
                            None => panic!(),
                            Some(nlr_arc) => {
                                let mut nlr = nlr_arc.lock().unwrap();
                                let nlr_diff = nlr.diff;
                                let diff = if nlr_diff == -1 {
                                    nlr.diff = 1;
                                    1
                                } else if nlr_diff == 0 {
                                    nlr.diff = 0;
                                    1
                                } else if nlr_diff == 1 {
                                    nlr.diff = 0;
                                    2
                                } else {
                                    panic!()
                                };
                                let nlr_val = nlr.value;
                                let nlr_n_ledu1 = nlr.n_ledu.1;
                                nlr.value = nl_val;
                                nlr.n_ledu.1 = nl_n_ledu1;
                                let nlrl_op = nlr.left.clone();
                                let nlrr_op = nlr.right.clone();
                                nlr.left = nll_op;
                                nlr.n_ledu.0 = match &nlr.left {
                                    Some(node) => node.lock().unwrap().len_child_and_self(),
                                    None => 0,
                                };
                                nlr.right = nlrl_op;
                                (nlr_val, nlr_n_ledu1, nlrr_op, diff)
                            }
                        };
                        nl.value = nlr_val;
                        nl.n_ledu.1 = nlr_n_ledu1;
                        let nlr_op = nl.right.clone();
                        nl.right = nlrr_op;
                        nl.left = nlr_op;
                        nl.n_ledu.0 = match &nl.left {
                            Some(node) => node.lock().unwrap().len_child_and_self(),
                            None => 0,
                        };
                        nl.diff = diff;
                    }
                    let n_diff = if nl.diff == 2 {
                        nl.diff = -1;
                        0
                    } else if nl.diff == 1 {
                        nl.diff = 0;
                        0
                    } else if nl.diff == 0 {
                        nl.diff = 1;
                        -1
                    } else {
                        panic!()
                    };
                    let nll_op = nl.left.clone();
                    nl.left = nl.right.clone();
                    nl.right = nr;
                    let nl_val = nl.value;
                    let nl_n_ledu1 = nl.n_ledu.1;
                    nl.value = n_val;
                    nl.n_ledu.1 = n_n_ledu1;
                    nl.n_ledu.0 = match &nl.left {
                        Some(node) => node.lock().unwrap().len_child_and_self(),
                        None => 0,
                    };
                    (nl_val, nl_n_ledu1, n_diff, nll_op)
                }
            };
            self.value = nl_val;
            self.n_ledu.1 = nl_n_ledu1;
            self.diff = n_diff;
            self.right = self.left.clone();
            self.left = nll_op;
            self.n_ledu.0 = match &self.left {
                Some(node) => node.lock().unwrap().len_child_and_self(),
                None => 0,
            };

            return true;
        } else if self.diff == -2 {
            let n_val = self.value;
            let n_n_ledu1 = self.n_ledu.1;
            let nl = self.left.clone();
            let (nr_val, nr_n_ledu1, n_diff, nrr_op) = match &mut self.right {
                None => {
                    panic!()
                }
                Some(nr_arc) => {
                    let mut nr = nr_arc.lock().unwrap();
                    let nr_val = nr.value;
                    let nr_n_ledu1 = nr.n_ledu.1;
                    let nrr_op = nr.right.clone();
                    if nr.diff == 1 {
                        let (nrl_val, nrl_n_ledu1, nrll_op, diff) = match &mut nr.left {
                            None => panic!(),
                            Some(nrl_arc) => {
                                let mut nrl = nrl_arc.lock().unwrap();
                                let nrl_diff = nrl.diff;
                                let diff = if nrl_diff == 1 {
                                    nrl.diff = -1;
                                    -1
                                } else if nrl_diff == 0 {
                                    nrl.diff = 0;
                                    -1
                                } else if nrl_diff == -1 {
                                    nrl.diff = 0;
                                    -2
                                } else {
                                    panic!()
                                };
                                let nrl_val = nrl.value;
                                let nrl_n_ledu1 = nrl.n_ledu.1;
                                nrl.value = nr_val;
                                nrl.n_ledu.1 = nr_n_ledu1;
                                let nrlr_op = nrl.right.clone();
                                let nrll_op = nrl.left.clone();
                                nrl.right = nrr_op;
                                nrl.left = nrlr_op;
                                nrl.n_ledu.0 = match &nrl.left {
                                    Some(node) => node.lock().unwrap().len_child_and_self(),
                                    None => 0,
                                };
                                (nrl_val, nrl_n_ledu1, nrll_op, diff)
                            }
                        };
                        nr.value = nrl_val;
                        nr.n_ledu.1 = nrl_n_ledu1;
                        let nrl_op = nr.left.clone();
                        nr.left = nrll_op;
                        nr.n_ledu.0 = match &nr.left {
                            Some(node) => node.lock().unwrap().len_child_and_self(),
                            None => 0,
                        };
                        nr.right = nrl_op;
                        nr.diff = diff;
                    }
                    let n_diff = if nr.diff == -2 {
                        nr.diff = 1;
                        0
                    } else if nr.diff == -1 {
                        nr.diff = 0;
                        0
                    } else if nr.diff == 0 {
                        nr.diff = -1;
                        1
                    } else {
                        panic!()
                    };
                    let nrr_op = nr.right.clone();
                    nr.right = nr.left.clone();
                    nr.left = nl;
                    let nr_val = nr.value;
                    let nr_n_ledu1 = nr.n_ledu.1;
                    nr.value = n_val;
                    nr.n_ledu.1 = n_n_ledu1;
                    nr.n_ledu.0 = match &nr.left {
                        Some(node) => node.lock().unwrap().len_child_and_self(),
                        None => 0,
                    };
                    (nr_val, nr_n_ledu1, n_diff, nrr_op)
                }
            };
            self.value = nr_val;
            self.n_ledu.1 = nr_n_ledu1;
            self.diff = n_diff;
            self.left = self.right.clone();
            self.right = nrr_op;
            self.n_ledu.0 = match &self.left {
                Some(node) => node.lock().unwrap().len_child_and_self(),
                None => 0,
            };

            return true;
        } else {
            panic!()
        }
    }

    /// Propagate the maximum value in the tree at the bottom to parent node.
    pub fn max_child(&self) -> T {
        match &self.right {
            Some(node) => node.lock().unwrap().max_child(),
            None => self.value,
        }
    }

    /// Return and remove the maximum value, 
    /// and propagate `(max_value, height_information)` to parent node.
    /// 
    /// If the maximum value is a duplicate, return and remove only one.
    pub fn pop_max_child(&mut self) -> Result<(T, DeltaDiff), ()> {
        let res = match &self.right {
            Some(node) => {
                let mut n = node.lock().unwrap();
                match &n.right {
                    Some(_) => {
                        match &n.pop_max_child() {
                            Ok((value, d_diff)) => Ok((*value, *d_diff, None)),
                            Err(()) => Err(()),
                        }
                    }
                    None => {
                        let value = n.value;
                        if n.n_ledu.1 > 0 {
                            n.n_ledu.1 -= 1;
                            Ok((value, DeltaDiff::Zero, None))
                        } else {
                            Ok((value, DeltaDiff::Shorter, Some(n.left.clone())))
                        }
                    }
                }
            }
            None => Err(()),
        };
        match res {
            Ok((value, d_diff, reconnect)) => {
                match reconnect {
                    Some(node_left) => { self.right = node_left; }
                    None => {}
                }
                Ok((value, self.balance(d_diff, Direction::Right)))
            }
            Err(()) => Err(()),
        }
    }

    /// Propagate `((max_value, number_of_duplicates), height_information)` to parent node, 
    /// then remove its node.
    pub fn pop_max_all_child(&mut self) -> Result<((T, usize), DeltaDiff), ()> {
        let res = match &self.right {
            Some(node) => {
                let mut n = node.lock().unwrap();
                match &n.right {
                    Some(_) => {
                        match &n.pop_max_all_child() {
                            Ok((value, d_diff)) => Ok((*value, *d_diff, None)),
                            Err(()) => Err(()),
                        }
                    }
                    None => {
                        let value = n.value;
                        Ok(((value, n.n_ledu.1), DeltaDiff::Zero, Some(n.left.clone())))
                    }
                }
            }
            None => Err(()),
        };
        match res {
            Ok((value, d_diff, reconnect)) => {
                match reconnect {
                    Some(node_left) => { self.right = node_left; }
                    None => {}
                }
                Ok((value, self.balance(d_diff, Direction::Right)))
            }
            Err(()) => Err(()),
        }
    }

    /// Propagate the maximum value in the tree at the bottom to parent node.
    pub fn min_child(&self) -> T {
        match &self.left {
            Some(node) => node.lock().unwrap().max_child(),
            None => self.value,
        }
    }

    /// Return and remove the maximum value, 
    /// and propagate `(max_value, height_information)` to parent node.
    /// 
    /// If the maximum value is a duplicate, return and remove only one.
    pub fn pop_min_child(&mut self) -> Result<(T, DeltaDiff), ()> {
        let res = match &self.left {
            Some(node) => {
                let mut n = node.lock().unwrap();
                match &n.left {
                    Some(_) => {
                        match &n.pop_min_child() {
                            Ok((value, d_diff)) => Ok((*value, *d_diff, None)),
                            Err(()) => Err(()),
                        }
                    }
                    None => {
                        let value = n.value;
                        if n.n_ledu.1 > 0 {
                            n.n_ledu.1 -= 1;
                            Ok((value, DeltaDiff::Zero, None))
                        } else {
                            Ok((value, DeltaDiff::Shorter, Some(n.right.clone())))
                        }
                    }
                }
            }
            None => Err(()),
        };
        match res {
            Ok((value, d_diff, reconnect)) => {
                match reconnect {
                    Some(node_right) => { self.left = node_right; }
                    None => {}
                }
                Ok((value, self.balance(d_diff, Direction::Left)))
            }
            Err(()) => Err(()),
        }
    }

    /// Propagate `((max_value, number_of_duplicates), height_information)` to parent node, 
    /// then remove its node.
    pub fn pop_min_all_child(&mut self) -> Result<((T, usize), DeltaDiff), ()> {
        let res = match &self.left {
            Some(node) => {
                let mut n = node.lock().unwrap();
                match &n.left {
                    Some(_) => {
                        match &n.pop_min_all_child() {
                            Ok((value, d_diff)) => Ok((*value, *d_diff, None)),
                            Err(()) => Err(()),
                        }
                    }
                    None => {
                        Ok(((n.value, n.n_ledu.1), DeltaDiff::Shorter, Some(n.right.clone())))
                    }
                }
            }
            None => Err(()),
        };
        match res {
            Ok((value, d_diff, reconnect)) => {
                match reconnect {
                    Some(node_right) => { self.left = node_right; }
                    None => {}
                }

                Ok((value, self.balance(d_diff, Direction::Left)))
            }
            Err(()) => Err(()),
        }
    }

    /// Return the number of elements in the tree at the bottom including itself.
    pub fn len_child_and_self(&self) -> usize {
        match &self.right {
            Some(node) => self.n_ledu.0 + self.n_ledu.1 + 1 + node.lock().unwrap().len_child_and_self(),
            None => self.n_ledu.0 + self.n_ledu.1 + 1,
        }
    }

    /// Return the height of the tree at the bottom.
    pub fn height_child(&self) -> usize {
        if self.diff >= 0 {
            match &self.left {
                Some(node) => 1 + node.lock().unwrap().height_child(),
                None => 1,
            }
        } else {
            match &self.right {
                Some(node) => 1 + node.lock().unwrap().height_child(),
                None => 1,
            }
        }
    }

    /// Utility function to test `diff`.
    pub fn check_diff(&self) {
        let hl = match &self.left {
            Some(node) => node.lock().unwrap().height_child(),
            None => 0,
        } as i32;
        let hr = match &self.right {
            Some(node) => node.lock().unwrap().height_child(),
            None => 0,
        } as i32;
        let diff = hl - hr;
        if diff != self.diff {
            panic!("Diff value is incorrect!: diff={}, left={}, right={}", self.diff, hl, hr);
        }
    }
}

