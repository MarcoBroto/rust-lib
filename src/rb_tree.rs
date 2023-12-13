
pub struct RBTree<T: Ord> {
	root: Option<RBTreeNode<T>>,
}

struct RBTreeNode<T: Ord> {
	val: T,
	isRed: bool,
}

impl<T: Ord> RBTree<T> {
	pub fn insert(&mut self, val: T) {
		// TODO: Implement insert value instance method
	}

	pub fn remove(&mut self, val: T) {
		// TODO: Implement delete value instance method
	}

	fn rotate_left(&mut self) {
		// TODO: 
	}

	fn rotate_right(&mut self) {
		// TODO: 
	}

	pub fn from_vec(vec: Vec<T>) -> RBTree<T> {
		// TODO: 
		let mut tree = RBTree::default();
		for x in vec { tree.insert(x); }
		tree
	}

	pub fn from_array(array: &[T]) -> RBTree<T> {
		let tree = RBTree::default();
		// TODO: 
		tree
	}


	pub fn is_valid_tree(tree: &RBTree<T>) {
		// TODO: Implement red black tree unit test
	}
}

impl<T: Ord> Default for RBTree<T> {
	fn default() -> RBTree<T> { RBTree { root: None } }
}

pub fn test_red_black_tree() {

}
