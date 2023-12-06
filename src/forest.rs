use std::{fmt::Debug, cmp::Ordering};

#[derive(Debug)]
pub struct Forest {
	nodes: Vec<ForestNode>,
}

#[derive(Debug, Clone, Copy)]
pub enum ForestNode {
	Root(i32),
	Child(usize)
}

impl Forest {
	pub fn create_forest(num_nodes: usize) -> Forest {
		Forest { nodes: vec![ForestNode::Root(-1); num_nodes] }
	}

	pub fn find(&mut self, ind: usize) -> ForestNode {
		let node = self.nodes.get(ind).unwrap_or_else(|| panic!("Vector index out of bounds on \"find\" operation."));

		match node {
			ForestNode::Root(_) => ForestNode::Child(ind),
			ForestNode::Child(x) => {
				if *x != ind {
					self.nodes[ind] = self.find(*x as usize);
				}
				return self.nodes[ind]
			}
		}
	}

	pub fn union(&mut self, u: usize, v: usize) {
		let (ForestNode::Child(u), ForestNode::Child(v)) = (self.find(u), self.find(v)) else { panic!("Could not cast enums to ForestNode::Child.") };

		if u == v { return }

		let (ForestNode::Root(rank_u), ForestNode::Root(rank_v)) = (self.nodes[u], self.nodes[v])else { panic!("Could not cast enums to ForestNode::Root.") };

		match rank_u.cmp(&rank_v) {
			Ordering::Less | Ordering::Equal => {
				self.nodes[v] = ForestNode::Child(u);
				self.nodes[u] = ForestNode::Root(rank_u-1);
			},
			Ordering::Greater => {
				self.nodes[u] = ForestNode::Child(v);
				self.nodes[v] = ForestNode::Root(rank_v-1);
			}
		}
	}

	pub fn to_int_vec(&self) -> Vec<i32> {
		let mut vec = Vec::new();

		for node in self.nodes.iter() {
			match *node {
				ForestNode::Root(x) => vec.push(x),
				ForestNode::Child(x) => vec.push(x as i32)
			}
		}

		return vec;
	}
}

pub fn test_forest() {
	let mut forest = Forest::create_forest(5);
    forest.union(1, 2);
	assert_eq!(forest.to_int_vec(), vec![-1, -2, 1, -1, -1]);
    forest.union(2, 4);
	assert_eq!(forest.to_int_vec(), vec![-1, -3, 1, -1, 1]);
    forest.union(1, 4);
	assert_eq!(forest.to_int_vec(), vec![-1, -3, 1, -1, 1]);
    forest.union(3, 0);
	assert_eq!(forest.to_int_vec(), vec![3, -3, 1, -2, 1]);
    forest.union(1, 1);
	assert_eq!(forest.to_int_vec(), vec![3, -3, 1, -2, 1]);
    forest.union(4, 3);
	assert_eq!(forest.to_int_vec(), vec![3, -4, 1, 1, 1]);
    forest.union(1, 0);
	assert_eq!(forest.to_int_vec(), vec![1, -4, 1, 1, 1]);
}
