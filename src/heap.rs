use std::cmp::Ord;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T: Ord + Debug> {
	tree: Vec<T>,
	size: usize,
	
}

impl<T: Ord + Debug> Default for Heap<T> {
	fn default() -> Heap<T> {
		Heap {
			tree: Vec::new(),
			size: 0,
		}
	}
}

impl<T: Ord + Debug> Heap<T> {
	pub fn heapify(array: &[T]) -> Heap<T> {
		println!("{:#?}", array);
		Heap::default()
	}

	pub fn peek(&self) -> &T { &self.tree[0] }

	pub fn insert(&mut self, val: T) {
		println!("{:#?}", self);

		self.size += 1;
		self.tree.push(val);
		let ind = self.size;
		while ind > 0 && self.tree[ind] < self.tree[ind/2] {
			self.tree.swap(ind, ind/2)
		}
	}

	pub fn remove(&mut self) {
		println!("{:#?}", self);
	}

	pub fn extract(&mut self) -> Option<T> {
		println!("{:#?}", self);

		if self.size > 0 {
			let val = self.tree.swap_remove(0);
			self.reorder();
			Some(val)
		}
		else { None }
	}

	fn reorder(&mut self) {
		let ind: usize = 0;
		while ind < self.size {
			let (lc, rc) = (ind*2, ind*2+1);
			if self.tree[ind] > self.tree[lc] {
				self.tree.swap(ind, lc);
			}
			else if self.tree[ind] > self.tree[rc] {
				self.tree.swap(ind, rc);
			}
		}
	}
}

pub fn test_heap() {
	let array: &[i32] = &[3,4,5,8,2];
    // let heap = create_heap(10);
    println!("{:?}", array);
}

