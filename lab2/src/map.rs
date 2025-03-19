use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
}

#[derive(Debug, Clone)]
pub struct Map<K: Ord, V> {
    root: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Map<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Map { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        Self::insert_node(&mut self.root, key, value);
    }

    fn insert_node(node: &mut Option<Rc<RefCell<Node<K, V>>>>, key: K, value: V) {
        match node {
            None => {
                *node = Some(Rc::new(RefCell::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                })));
            }
            Some(n) => {
                let mut n_ref = n.borrow_mut();
                if key < n_ref.key {
                    Self::insert_node(&mut n_ref.left, key, value);
                } else if key > n_ref.key {
                    Self::insert_node(&mut n_ref.right, key, value);
                } else {
                    n_ref.value = value;
                }
            }
        }
    }

    pub fn find(&self, key: &K) -> Option<Rc<RefCell<Node<K, V>>>> {
        Self::find_node(&self.root, key)
    }

    fn find_node(
        node: &Option<Rc<RefCell<Node<K, V>>>>,
        key: &K,
    ) -> Option<Rc<RefCell<Node<K, V>>>> {
        match node {
            None => None,
            Some(n) => {
                let n_ref = n.borrow();
                if key < &n_ref.key {
                    Self::find_node(&n_ref.left, key)
                } else if key > &n_ref.key {
                    Self::find_node(&n_ref.right, key)
                } else {
                    Some(n.clone())
                }
            }
        }
    }
}

impl<K, V> Drop for Map<K, V>
where
    K: Ord,
{
    fn drop(&mut self) {
        fn cleanup<K: Ord, V>(node: Option<Rc<RefCell<Node<K, V>>>>) {
            if let Some(n) = node {
                let mut n_ref = n.borrow_mut();
                cleanup(n_ref.left.take());
                cleanup(n_ref.right.take());
            }
        }
        cleanup(self.root.take());
    }
}
