
pub struct RedBlackTree<T: Ord> {
	root: RbTreeNode<T>,
}

struct RbTreeNode {
	val: T,
	isRed: bool,
}

impl<T: Ord> RedBlackTree<T> {
	pub fn insert(&self, val: T) {
		// TODO: Implement insert value instance method
	}

	pub fn remove(&self, val: T) {
		// TODO: Implement delete value instance method
	}
}

impl<T: Ord> Default for RedBlackTree<T> {
	fn new() {

	}
}

pub fn test_red_black_tree() {
	// TODO: Implement red black tree unit test
}
