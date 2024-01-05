use std::cmp::Ordering;

type RBTreeNodeType<K> = Option<Box<RBTreeNode<K>>>;

#[derive(Debug)]
struct RBTreeNode<K: Ord> {
	key: K,
	left: RBTreeNodeType<K>,
	right: RBTreeNodeType<K>,
	is_black: bool,
}

impl<K: Ord> RBTreeNode<K> {
	pub fn new(key: K, is_black: bool) -> Self {
		RBTreeNode {
			key,
			left: None,
			right: None,
			is_black,
		}
	}
}

#[derive(Debug)]
pub struct RBTree<K: Ord> {
	root: RBTreeNodeType<K>,
}

impl<K: Ord> RBTree<K> {
	pub fn insert(&mut self, key: K) {
		if self.root.is_none() { // Tree does not contain any elements
			self.root = Some(Box::new(RBTreeNode::new(key, true)));
			return
		}

		let mut head = &mut self.root;

		while let Some(ref mut curr) = head {
			match curr.key.cmp(&key) {
				Ordering::Equal => return, // Key already exists in tree
				Ordering::Greater => {
					match curr.left {
						Some(_) => { head = &mut curr.left; },
						None => {
							curr.left = Some(Box::new(RBTreeNode::new(key, false)));
							break;
						},
					}
				},
				Ordering::Less => {
					match &curr.right {
						Some(_) => { head = &mut curr.right; },
						None => {
							curr.right = Some(Box::new(RBTreeNode::new(key, false)));
							break;
						},
					}
				}
			}
		}

		self.rebalance()
	}

	pub fn remove(&mut self, _: K) {
		// TODO: Implement delete value instance method
	}

	fn rotate_left(&mut self, pivot: RBTreeNode<K>) {
		// TODO: Implement left rotation instance method
	}

	fn rotate_right(&mut self, pivot: RBTreeNode<K>) {
		// TODO: Implement right rotation instance method
	}

	fn rebalance(&mut self) {
		// TODO: Implement rebalance tree instance method
	}

	pub fn from_vec(vec: Vec<K>) -> RBTree<K> {
		let mut tree = RBTree::default();
		for x in vec { tree.insert(x); }
		tree
	}

	pub fn from_array(_: &[K]) -> RBTree<K> {
		let tree = RBTree::default();
		// TODO: Implement RB Tree from array initializer
		tree
	}

	pub fn is_valid_tree(tree: &RBTree<K>) -> bool {
		if let Some(root) = &tree.root {
			if !root.is_black { return false }

			// Depth-first-search through tree to find violations
			let mut stack: Vec<&Box<RBTreeNode<K>>> = vec![root];
			while !stack.is_empty() {
				let curr = stack.pop().unwrap();
				if let Some(left) = &curr.left {
					if !curr.is_black && !left.is_black { return false }
					stack.push(&left);
				}
				if let Some(right) = &curr.right {
					if !curr.is_black && !right.is_black { return false }
					stack.push(&right);
				}
			}
		}
		true
	}
}

impl<T: Ord> Default for RBTree<T> {
	fn default() -> RBTree<T> { RBTree { root: None } }
}
