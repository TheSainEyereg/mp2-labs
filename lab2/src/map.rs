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

    pub fn get(&self, key: &K) -> Option<Rc<RefCell<Node<K, V>>>> {
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

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn iter(&self) -> MapIterator<K, V> {
        let mut iter = MapIterator { stack: Vec::new() };
        let mut current = self.root.clone();
        while let Some(node) = current {
            iter.stack.push(node.clone());
            current = node.borrow().left.clone();
        }
        iter
    }
}

pub struct MapIterator<K: Ord, V> {
    stack: Vec<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Iterator for MapIterator<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let node_ref = node.borrow();
            let result = (node_ref.key.clone(), node_ref.value.clone());

            let mut current = node_ref.right.clone();
            while let Some(next_node) = current {
                self.stack.push(next_node.clone());
                current = next_node.borrow().left.clone();
            }

            Some(result)
        } else {
            None
        }
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = MapIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
