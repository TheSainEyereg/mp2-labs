#[derive(Debug, Clone)]
pub struct Node<K: Ord, V: Clone> {
    pub key: K,
    pub value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone)]
pub struct Map<K: Ord, V: Clone> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Map { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        Self::insert_node(&mut self.root, key, value);
    }

    fn insert_node(node: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        match node {
            None => {
                *node = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                }));
            }
            Some(n) => {
                if key < n.key {
                    Self::insert_node(&mut n.left, key, value);
                } else if key > n.key {
                    Self::insert_node(&mut n.right, key, value);
                } else {
                    n.value = value;
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(node) = Self::find_node(&self.root, key) {
            Some(node.value)
        } else {
            None
        }
    }

    fn find_node(node: &Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        match node {
            None => None,
            Some(n) => {
                if key < &n.key {
                    Self::find_node(&n.left, key)
                } else if key > &n.key {
                    Self::find_node(&n.right, key)
                } else {
                    Some(n.clone())
                }
            }
        }
    }

    pub fn find(&self, key: &K) -> MapIterator<K, V> {
        let mut iter = MapIterator { stack: Vec::new() };
        let mut current = Self::find_node(&self.root, key);
        while let Some(node) = current {
            iter.stack.push(node.clone());
            current = node.left.clone();
        }
        iter
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn iter(&self) -> MapIterator<K, V> {
        let mut iter = MapIterator { stack: Vec::new() };
        let mut current = self.root.clone();
        while let Some(node) = current {
            iter.stack.push(node.clone());
            current = node.left.clone();
        }
        iter
    }
}

pub struct MapIterator<K: Ord, V: Clone> {
    stack: Vec<Box<Node<K, V>>>,
}

impl<K, V> Iterator for MapIterator<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let result = (node.key.clone(), node.value.clone());

            let mut current = node.right.clone();
            while let Some(next_node) = current {
                self.stack.push(next_node.clone());
                current = next_node.left.clone();
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
