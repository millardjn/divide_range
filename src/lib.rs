
extern crate num_traits;

use std::ops::Range;
use num_traits::{Num, FromPrimitive};



trait RangeDivisions<T: Num + FromPrimitive + PartialOrd + Copy> {
	fn divide_evenly_into(self, divisions: usize) -> Even<T>;

	// fn divide_greedily_into(self, divisions: usize) -> Greedy<T>;
}

impl<T: Num + FromPrimitive + PartialOrd + Copy> RangeDivisions<T> for Range<T> {
	fn divide_evenly_into(self, divisions: usize) -> Even<T> {
		Even {
			next_start: self.start,
			next_end: self.start,
			end: self.end,
			div_remaining: divisions,
		}
	}

	// fn divide_greedily_into(self, divisions: usize) -> Greedy<T>{
	// 	Greedy {
	// 		start: self.start,
	// 		end: self.end,
	// 		chunk_size: if self.end > self.start {self.end - self.start} else {self.start - self.end}/T::from_usize(divisions).expect("divisions usize cannot be converted to range type"),
	// 	}
	// }
}


struct Even<T: Num + FromPrimitive + PartialOrd + Copy> {
	next_start: T,
	next_end: T,
	end: T,
	div_remaining: usize,
}


impl<T: Num + FromPrimitive + PartialOrd + Copy> Iterator for Even<T> {
	type Item = Range<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.div_remaining > 0 {
			self.next_start = self.next_end;
			if self.next_start < self.end {
				self.next_end = self.next_start + (self.end - self.next_start)/T::from_usize(self.div_remaining).expect("divisions usize cannot be converted to range type");
			} else {
				self.next_end = self.next_start - (self.next_start - self.end)/T::from_usize(self.div_remaining).expect("divisions usize cannot be converted to range type");
			}
			
			self.div_remaining -= 1;

			Some(self.next_start..self.next_end)
		} else {
			None
		}

	}
}


// struct Greedy<T: Num + PartialOrd + Copy> {
// 	start: T,
// 	end: T,
// 	chunk_size: T,
// }


// impl<T: Num + PartialOrd + Copy> Iterator for Greedy<T> {
// 	type Item = Range<T>;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		if self.start < self.end {
// 			let previous = self.start;
// 			self.start = if self.start + self.chunk_size < self.end {
// 				self.start + self.chunk_size
// 			} else {
// 				self.end
// 			};
			
// 			Some(previous..self.start)
// 		} else if self.start > self.end {
// 			let previous = self.start;
// 			self.start = if self.start - self.chunk_size > self.end {
// 				self.start - self.chunk_size
// 			} else {
// 				self.end
// 			};
			
// 			Some(previous..self.start)
// 		} else {
// 			None
// 		}

// 	}
// }


#[cfg(test)]
mod tests {

	use super::RangeDivisions;
	
	#[test]
	fn even_forward() {
		let range = 1..18;
		let mut iter = range.divide_evenly_into(5);

		assert_eq!(Some(1..4), iter.next());
		assert_eq!(Some(4..7), iter.next());
		assert_eq!(Some(7..10), iter.next());
		assert_eq!(Some(10..14), iter.next());
		assert_eq!(Some(14..18), iter.next());
		assert_eq!(None, iter.next());
	}

	#[test]
	fn even_backward() {
		let range = 18..1;
		let mut iter = range.divide_evenly_into(5);

		assert_eq!(Some(18..15), iter.next());
		assert_eq!(Some(15..12), iter.next());
		assert_eq!(Some(12..9), iter.next());
		assert_eq!(Some(9..5), iter.next());
		assert_eq!(Some(5..1), iter.next());
		assert_eq!(None, iter.next());
	}


	// #[test]
	// fn greedily_forward() {
	// 	let range = 1..18;
	// 	let mut iter = range.divide_greedily_into(5);

	// 	assert_eq!(Some(1..5), iter.next());
	// 	assert_eq!(Some(5..9), iter.next());
	// 	assert_eq!(Some(9..13), iter.next());
	// 	assert_eq!(Some(13..17), iter.next());
	// 	assert_eq!(Some(17..18), iter.next());
	// 	assert_eq!(None, iter.next());
	// }

	// #[test]
	// fn greedily_backward() {
	// 	let range = 18..1;
	// 	let mut iter = range.divide_greedily_into(5);

	// 	assert_eq!(Some(18..14), iter.next());
	// 	assert_eq!(Some(14..10), iter.next());
	// 	assert_eq!(Some(10..6), iter.next());
	// 	assert_eq!(Some(6..2), iter.next());
	// 	assert_eq!(Some(2..1), iter.next());
	// 	assert_eq!(None, iter.next());
	// }
}
