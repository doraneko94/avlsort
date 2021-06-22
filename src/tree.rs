//! AVL tree.
 
use crate::node::AvlNode;
use crate::traits::TreeElem;

/// AVL tree.
pub struct AvlTree<T> {
    /// Root node.
    pub root: Option<AvlNode<T>>,
}

impl<T: TreeElem> AvlTree<T> {
    /// Create an empty AVL tree.
    pub fn new() -> Self {
        let root = None;
        Self { root }
    }

    /// Push `value` and return the rank and the number of duplication of it.
    pub fn push(&mut self, value: T) -> (usize, usize) {
        match &mut self.root {
            Some(r) => {
                let (n_ledu, _) = r.push_child(value);
                n_ledu
            }
            None => {
                self.root = Some(AvlNode::new(value));
                (0, 0)
            }
        }
    }

    /// Determine if `value` exists.
    pub fn isin(&self, value: T) -> bool {
        match match &self.root {
            Some(r) => r.search(value),
            None => None,
        } {
            Some(_) => true,
            None => false,
        }
    }

    /// Count the number of `value`.
    pub fn count(&self, value: T) -> usize {
        match match &self.root {
            Some(r) => r.search(value),
            None => None,
        } {
            Some(dup) => dup + 1,
            None => 0,
        }
    }

    /// Remove `value` from the tree and return the result.
    /// 
    /// If `value` is a duplicate, return and remove only one.
    pub fn remove(&mut self, value: T) -> Result<(), ()> {
        match &mut self.root {
            Some(r) => {
                if r.value == value {
                    if r.n_ledu.1 > 0 {
                        r.n_ledu.1 -= 1;
                        Ok(())
                    } else {
                        let res = match (&r.left, &r.right) {
                            (Some(nl), Some(nr)) => {
                                let n_m = if r.diff >= 0 {
                                    nl.lock().unwrap().max_child()
                                } else {
                                    nr.lock().unwrap().min_child()
                                };
                                r.value = n_m;
                                r.n_ledu.1 = r.search(n_m).unwrap();
                                match r.remove_child(n_m) {
                                    Ok(_) => {}
                                    Err(()) => panic!(),
                                }
                                Ok(None)
                            }
                            (Some(nl), None) => {
                                let nl_lock = nl.lock().unwrap();
                                r.value = nl_lock.value;
                                r.diff = nl_lock.diff;
                                r.n_ledu = nl_lock.n_ledu;
                                Ok(Some((nl_lock.left.clone(), nl_lock.right.clone())))
                            }
                            (None, Some(nr)) => {
                                let nr_lock = nr.lock().unwrap();
                                r.value = nr_lock.value;
                                r.diff = nr_lock.diff;
                                r.n_ledu = nr_lock.n_ledu;
                                Ok(Some((nr_lock.left.clone(), nr_lock.right.clone())))
                            }
                            (None, None) => {
                                Err(())
                            }
                        };
                        match res {
                            Ok(Some((nl_op, nr_op))) => {
                                r.left = nl_op;
                                r.right = nr_op;
                            }
                            Ok(None) => {}
                            Err(()) => {
                                self.root = None;
                            }
                        }
                        Ok(())
                    }
                } else {
                    let res = r.remove_child(value);
                    match res {
                        Ok(_) => Ok(()),
                        Err(()) => Err(()),
                    }
                }
            }
            None => Err(())
        }
    }

    /// Return the maximum value in the tree.
    pub fn max(&self) -> Option<T> {
        match &self.root {
            Some(r) => Some(r.max_child()),
            None => None,
        }
    }

    /// Return and remove the maximum value.
    /// 
    /// If the maximum value is a duplicate, return and remove only one.
    pub fn pop_max(&mut self) -> Option<T> {
        match &mut self.root {
            Some(r) => { 
                let res = match &r.right {
                    Some(_) => true,
                    None => false,
                };
                if res {
                    let (value, _) = r.pop_max_child().unwrap();
                    Some(value)
                } else {
                    let ret_value = r.value;
                    if r.n_ledu.1 > 0 {
                        r.n_ledu.1 -= 1;
                    } else {
                        let op = match &r.left {
                            Some(node) => {
                                let n = node.lock().unwrap();
                                Some((n.value, n.diff, n.n_ledu, n.left.clone(), n.right.clone()))
                            }
                            None => None,
                        };
                        match op {
                            Some((value, diff, n_ledu, left, right)) => {
                                r.value = value;
                                r.diff = diff;
                                r.n_ledu = n_ledu;
                                r.left = left;
                                r.right = right;
                            }
                            None => { self.root = None; } 
                        }
                    }
                    Some(ret_value)
                }
            }
            None => None,
        }
    }

    /// Return the maximum value and the number of duplication of it, then remove its node.
    pub fn pop_max_all(&mut self) -> Option<(T, usize)> {
        match &mut self.root {
            Some(r) => { 
                let res = match &r.right {
                    Some(_) => true,
                    None => false,
                };
                if res {
                    let (value, _) = r.pop_max_all_child().unwrap();
                    Some(value)
                } else {
                    let ret_value = (r.value, r.n_ledu.1);
                    let op = match &r.left {
                        Some(node) => {
                            let n = node.lock().unwrap();
                            Some((n.value, n.diff, n.n_ledu, n.left.clone(), n.right.clone()))
                        }
                        None => None,
                    };
                    match op {
                        Some((value, diff, n_ledu, left, right)) => {
                            r.value = value;
                            r.diff = diff;
                            r.n_ledu = n_ledu;
                            r.left = left;
                            r.right = right;
                        }
                        None => { self.root = None; } 
                    }
                    Some(ret_value)
                }
            }
            None => None,
        }
    }
 
    /// Return the maximum value in the tree.
    pub fn min(&self) -> Option<T> {
        match &self.root {
            Some(r) => Some(r.min_child()),
            None => None,
        }
    }

    /// Return and remove the maximum value.
    /// 
    /// If the maximum value is a duplicate, return and remove only one.
    pub fn pop_min(&mut self) -> Option<T> {
        match &mut self.root {
            Some(r) => { 
                let res = match &r.left {
                    Some(_) => true,
                    None => false,
                };
                if res {
                    let (value, _) = r.pop_min_child().unwrap();
                    Some(value)
                } else {
                    let ret_value = r.value;
                    if r.n_ledu.1 > 0 {
                        r.n_ledu.1 -= 1;
                    } else {
                        let op = match &r.right {
                            Some(node) => {
                                let n = node.lock().unwrap();
                                Some((n.value, n.diff, n.n_ledu, n.left.clone(), n.right.clone()))
                            }
                            None => None,
                        };
                        match op {
                            Some((value, diff, n_ledu, left, right)) => {
                                r.value = value;
                                r.diff = diff;
                                r.n_ledu = n_ledu;
                                r.left = left;
                                r.right = right;
                            }
                            None => { self.root = None; } 
                        }
                    }
                    Some(ret_value)
                }
            }
            None => None,
        }
    }

    /// Return the maximum value and the number of duplication of it, then remove its node.
    pub fn pop_min_all(&mut self) -> Option<(T, usize)> {
        match &mut self.root {
            Some(r) => {
                let res = match &r.left {
                    Some(_) => true,
                    None => false,
                };
                if res {
                    let (value, _) = r.pop_min_all_child().unwrap();
                    Some(value)
                } else {
                    let ret_value = (r.value, r.n_ledu.1);
                    let op = match &r.right {
                        Some(node) => {
                            let n = node.lock().unwrap();
                            Some((n.value, n.diff, n.n_ledu, n.left.clone(), n.right.clone()))
                        }
                        None => None,
                    };
                    match op {
                        Some((value, diff, n_ledu, left, right)) => {
                            r.value = value;
                            r.diff = diff;
                            r.n_ledu = n_ledu;
                            r.left = left;
                            r.right = right;
                        }
                        None => { self.root = None; } 
                    }
                    Some(ret_value)
                }
            }
            None => None,
        }
    }

    /// Return the number of elements.
    pub fn len(&self) -> usize {
        match &self.root {
            Some(r) => r.len_child_and_self(),
            None => 0,
        }
    }

    /// Return the maximum height of the tree.
    pub fn height(&self) -> usize {
        match &self.root {
            Some(r) => r.height_child(),
            None => 0,
        }
    }
}