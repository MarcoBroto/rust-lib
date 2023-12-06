use std::cmp::Ordering;

pub fn find<T: Ord, V: AsRef<[T]>>(sorted_array: V, key: T) -> Option<usize> {
	let sorted_array = sorted_array.as_ref();
	let (mut l, mut r) = (0, sorted_array.len());

	while l < r {
		let mid = (l + r) / 2;
		match sorted_array[mid].cmp(&key) {
			Ordering::Less => l = mid+1,
			Ordering::Greater => r = mid,
			Ordering::Equal => return Some(mid)
		}
	}

	return None
}

pub fn test_binary_search() {
    let arr = [-3,0,1,2,3,6];
    assert_eq!(find(arr, -3), Some(0));
    assert_eq!(find(arr, -2), None);
}
