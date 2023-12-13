use std::cmp::{Ord, Ordering};
use std::fmt::{Debug, Display};

pub struct Heap<T: Ord> {
	tree: Vec<T>,
	size: usize,
}

impl<T: Ord> Default for Heap<T> {
	fn default() -> Heap<T> {
		Heap {
			tree: Vec::new(),
			size: 0,
		}
	}
}

impl<T: Ord + Debug> Display for Heap<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.tree)
	}
}

impl<T: Ord + Copy + Debug> Heap<T> {
	pub fn heapify_array(array: &[T]) -> Heap<T> { Self::heapify_vec(array.to_vec()) }

	pub fn heapify_vec(vec: Vec<T>) -> Heap<T> {
		let (len, mut ind) = (vec.len(), vec.len() / 2);
		let mut heap= Heap { tree: vec, size: len, };

		while ind > 0 {
			heap.percolate(ind);
			ind -= 1;
		}
		heap.percolate(0);

		heap
	}
}

impl<T: Ord> Heap<T> {
	pub fn peek(&self) -> &T { &self.tree[0] }

	pub fn insert(&mut self, val: T) {
		self.tree.push(val);
		let ind: usize = self.tree.len()-1;
		while ind > 0 && self.tree[ind] < self.tree[(ind-1)/2] {
			self.tree.swap(ind, (ind-1)/2)
		}
	}

	pub fn remove(&mut self) {
		// TODO: Implement remove element from heap instance method
	}

	pub fn extract(&mut self) -> Option<T> {
		if self.size > 0 {
			let val = self.tree.swap_remove(0);
			self.percolate(0);
			Some(val)
		}
		else { None }
	}

	fn percolate(&mut self, mut ind: usize) {
		let nl_ind = self.tree.len() / 2;
		while ind <= nl_ind {
			let (lc_ind, rc_ind) = (ind*2, ind*2+1);
			if rc_ind < self.tree.len() { // Right child exists
				let swap_ind = match self.tree[rc_ind].cmp(&self.tree[lc_ind]) {
					Ordering::Less | Ordering::Equal => rc_ind,
					Ordering::Greater => lc_ind,
				};
				if self.tree[ind] > self.tree[swap_ind] {
					self.tree.swap(ind, swap_ind);
					ind = swap_ind;
				} else { break; }
			}
			else { // Only left child exists
				if self.tree[ind] > self.tree[lc_ind] {
					self.tree.swap(ind, lc_ind);
					ind = lc_ind;
				} else { break; }
			}
		}
	}

	pub fn is_valid_heap(heap: &Heap<T>) -> bool {
	for i in 0..((heap.tree.len()-1)/2) {
		let (lc_ind, rc_ind) = ((i+1)*2, (i+1)*2+1);
		if heap.tree[i] > heap.tree[lc_ind] { return false }
		else if rc_ind < heap.tree.len() && heap.tree[i] > heap.tree[rc_ind] { return false }
	}
	true
}
}

pub fn test_heap() {
	let heap = Heap::heapify_array(&[3,4,5,8,2]);
	assert!(Heap::is_valid_heap(&heap));
	let mut heap = Heap::heapify_array(&[9,8,7,6,5,4,3]);
	assert!(Heap::is_valid_heap(&heap));
	println!("{}", heap);
	heap.insert(11);
	heap.insert(4);
	heap.insert(5);
	heap.insert(9);
	heap.insert(10);
	println!("{}", heap);
	assert!(Heap::is_valid_heap(&heap));
}
