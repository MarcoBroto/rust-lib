use std::cmp::Ordering;

type RBTreeNodeType<T> = Option<Box<RBTreeNode<T>>>; // Type alias for wrapped Red Black Tree Node

enum RBTreeRotation { LL, LR, RR, RL }

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

		// Traverse tree and insert new node if key does not already exist in tree
		let mut rotations: Vec<RBTreeRotation> = Vec::default();
		let mut path: Vec<&mut RBTreeNodeType<K>> = Vec::default();

		let mut head = &mut self.root; // Stores current traversal node position
		while let Some(ref mut curr) = head {
			match curr.key.cmp(&key) {
				Ordering::Equal => return, // Key already exists in tree
				Ordering::Greater => // Current node is greater than insertion key, traverse left
					match curr.left {
						Some(_) => { head = &mut curr.left; }, // Traverse existing right child branch path
						None => { // No child exists, insert key
							curr.left = Some(Box::new(RBTreeNode::new(key, false)));
							break;
						},
					},
				Ordering::Less =>  // Current node is less than insertion key, traverse right
					match &curr.right {
						Some(_) => { head = &mut curr.right; }, // Traverse existing left child branch path
						None => { // No child exists, insert key
							curr.right = Some(Box::new(RBTreeNode::new(key, false)));
							break;
						},
					}
			}
		}

		// TODO: Fix tree violations
	}

	pub fn remove(&mut self, _: K) {
		// TODO: Implement delete value instance method
	}

	fn rotate(&mut self, rotation: RBTreeRotation ) {
		// TODO: Implement rotation method

		match rotation {
			RBTreeRotation::LL => {},
			RBTreeRotation::LR => {},
			RBTreeRotation::RR => {},
			RBTreeRotation::RL => {},
		}
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
		// TODO: Implement black node depth check validation
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
