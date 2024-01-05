
#[cfg(test)]
mod tests {
    use crate::RBTree;
	
	#[test]
	fn test_rb_tree_insertions() {
		let mut tree = RBTree::default();
		tree.insert(2);
		tree.insert(1);
		tree.insert(4);
		tree.insert(3);
		println!("{:?}", tree);
	}
}
