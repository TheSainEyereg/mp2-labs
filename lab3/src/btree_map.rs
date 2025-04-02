use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Node<K: Ord, V: Clone> {
    leaf: bool,
    keys: Vec<(K, V)>,
    children: Vec<Node<K, V>>,
}

#[derive(Debug, Clone)]
pub struct BtreeMap<K: Ord, V: Clone> {
    t: usize,
    root: Option<Node<K, V>>,
}

impl<K: Ord, V: Clone> BtreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn new(t: usize) -> Self {
        BtreeMap { t, root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let new_root = Node {
                leaf: true,
                keys: vec![(key, value)],
                children: Vec::new(),
            };
            self.root = Some(new_root);
            return;
        }

        let mut root = self.root.take().unwrap();
        if root.keys.len() == 2 * self.t - 1 {
            let mut new_root = Node {
                leaf: false,
                keys: Vec::new(),
                children: vec![root],
            };
            self.split_child(&mut new_root, 0);
            self.insert_non_full(&mut new_root, key, value);
            self.root = Some(new_root);
        } else {
            self.insert_non_full(&mut root, key, value);
            self.root = Some(root);
        }
    }

    fn insert_non_full(&self, node: &mut Node<K, V>, key: K, value: V) {
        let mut i = node.keys.len() as isize - 1;

        if node.leaf {
            node.keys.push((key, value));
            node.keys.sort_by(|a, b| a.0.cmp(&b.0));
        } else {
            while i >= 0 && node.keys[i as usize].0 > key {
                i -= 1;
            }
            let child_idx = (i + 1) as usize;

            if node.children[child_idx].keys.len() == 2 * self.t - 1 {
                self.split_child(node, child_idx);
                if node.keys[child_idx].0 < key {
                    i += 1;
                }
            }
            self.insert_non_full(&mut node.children[(i + 1) as usize], key, value);
        }
    }

    fn split_child(&self, parent: &mut Node<K, V>, child_idx: usize) {
        let t = self.t;
        let child = &mut parent.children[child_idx];
        let median_idx = t - 1;

        let mut new_node = Node {
            leaf: child.leaf,
            keys: child.keys.split_off(median_idx + 1),
            children: Vec::new(),
        };

        if !child.leaf {
            new_node.children = child.children.split_off(t);
        }

        let median_key = child.keys.pop().unwrap();
        parent.keys.insert(child_idx, median_key);
        parent.children.insert(child_idx + 1, new_node);
    }

    fn find_node<'a>(node: Option<&'a Node<K, V>>, key: &K) -> Option<(&'a Node<K, V>, usize)> {
        match node {
            None => None,
            Some(n) => {
                for (i, (k, _)) in n.keys.iter().enumerate() {
                    if k == key {
                        return Some((n, i));
                    }
                    if key < k {
                        return if n.leaf {
                            None
                        } else {
                            Self::find_node(n.children.get(i).map(|c| c), key)
                        };
                    }
                }
                if !n.leaf {
                    Self::find_node(n.children.last().map(|c| c), key)
                } else {
                    None
                }
            }
        }
    }

    pub fn find(&self, key: &K) -> BtreeIterator<K, V> {
        let mut iter = BtreeIterator { stack: Vec::new() };

        if let Some(root) = &self.root {
            if let Some((node, index)) = Self::find_node(Some(root), key) {
                iter.stack.push((node.clone(), index));

                if !node.leaf && index + 1 < node.children.len() {
                    let mut current = node.children[index + 1].clone();
                    while !current.leaf {
                        iter.stack.push((current.clone(), 0));
                        current = current.children[0].clone();
                    }
                    iter.stack.push((current, 0));
                }
            }
        }

        iter
    }

    pub fn iter(&self) -> BtreeIterator<K, V> {
        let mut iter = BtreeIterator { stack: Vec::new() };

        if let Some(root) = &self.root {
            let mut current = root.clone();
            while !current.leaf {
                iter.stack.push((current.clone(), 0));
                current = current.children[0].clone();
            }
            iter.stack.push((current, 0));
        }

        iter
    }
}

#[derive(Debug)]
pub struct BtreeIterator<K: Ord, V: Clone> {
    stack: Vec<(Node<K, V>, usize)>,
}

impl<K, V> Iterator for BtreeIterator<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, mut index)) = self.stack.pop() {
            if index < node.keys.len() {
                let result = node.keys[index].clone();

                if !node.leaf && index < node.children.len() {
                    let mut current = node.children[index + 1].clone();
                    while !current.leaf {
                        self.stack.push((current.clone(), 0));
                        current = current.children[0].clone();
                    }
                    self.stack.push((current, 0));
                }

                index += 1;
                if index < node.keys.len() || !node.leaf {
                    self.stack.push((node, index));
                }

                return Some(result);
            }
        }
        None
    }
}

impl<K, V> IntoIterator for BtreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = BtreeIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> Index<K> for BtreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        let (node, i) =
            Self::find_node(self.root.as_ref(), &index).expect("Key {index} out of bounds");

        &node.keys[i].1
    }
}
