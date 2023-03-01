use console::Emoji;
use core::cmp::Ordering;
use dialoguer::{theme::ColorfulTheme, Select};

use rand::Rng;
use std::time::Instant;

const SIZE: usize = 10_000;

struct Algs {
	array: [i32; SIZE],
}

impl Algs {
	fn new() -> Algs {
		let length = rand::thread_rng().gen_range(0..SIZE);
		let mut result: [i32; SIZE] = [0; SIZE];
		for i in 0..length {
			result[i] = rand::thread_rng().gen_range(-(SIZE as i32)..SIZE as i32) as i32;
		}
		Algs { array: result }
	}

	fn bubble(&self) -> [i32; SIZE] {
		let mut array = self.array.to_owned();
		for i in 0..array.len() {
			for j in 0..array.len() - 1 - i {
				if array[j] > array[j + 1] {
					array.swap(j, j + 1);
				}
			}
		}
		array
	}

	fn select(&self) -> [i32; SIZE] {
		let mut array = self.array.to_owned();

		for i in 0..array.len() {
			let mut min = i;
			for j in i + 1..array.len() {
				if array[min] > array[j] {
					min = j
				}
			}
			array.swap(i, min);
		}

		array
	}

	fn merge_sort(&self) -> [i32; SIZE] {
		fn merge(mut arr: [i32; SIZE], left: usize, mid: usize, right: usize) -> [i32; SIZE] {
			let n1 = mid - left;
			let n2 = right - mid;
			let l1 = arr.clone();
			let r1 = arr.clone();
			let l = &l1[left..mid];
			let r = &r1[mid..right];
			let mut i = 0;
			let mut j = 0;
			let mut k = left;
			while i < n1 && j < n2 {
				if l[i] < r[j] {
					arr[k] = l[i];
					i += 1;
				} else {
					arr[k] = r[j];
					j += 1;
				}
				k += 1;
			}
			while i < n1 {
				arr[k] = l[i];
				i += 1;
				k += 1;
			}
			while j < n2 {
				arr[k] = r[j];
				j += 1;
				k += 1;
			}
			arr
		}

		fn sort(mut arr: [i32; SIZE], left: usize, right: usize) -> [i32; SIZE] {
			if right - 1 > left {
				let mid = left + (right - left) / 2;
				arr = sort(arr, left, mid);
				arr = sort(arr, mid, right);
				arr = merge(arr, left, mid, right);
			}
			arr
		}

		sort(self.array.clone(), 0, self.array.len())
	}

	fn radix(&self) -> [i32; SIZE] {
		let mut array = self.array.to_owned();

		let max = *array.iter().max().unwrap() as usize;

		// Make radix a power of 2 close to array.len() for optimal runtime
		let radix = array.len().next_power_of_two();
		// Counting sort by each digit from least to most significant
		let mut place = 1;
		while place <= max {
			let digit_of = |x| x as usize / place % radix;
			// Count digit occurrences
			let mut counter = vec![0; radix];
			for &x in array.iter() {
				counter[digit_of(x)] += 1;
			}
			// Compute last index of each digit
			for i in 1..radix {
				counter[i] += counter[i - 1];
			}
			// Write elements to their new indices
			for &x in array.to_owned().iter().rev() {
				counter[digit_of(x)] -= 1;
				array[counter[digit_of(x)]] = x;
			}
			place *= radix;
		}
		array
	}

	fn quick(&self) -> [i32; SIZE] {
		let mut array = self.array.to_owned();

		fn quicksort_helper<T, F>(arr: &mut [T], left: isize, right: isize, compare: &F)
		where
			F: Fn(&T, &T) -> Ordering,
		{
			if right <= left {
				return;
			}

			let mut i: isize = left - 1;
			let mut j: isize = right;
			let mut p: isize = i;
			let mut q: isize = j;
			unsafe {
				let v: *mut T = &mut arr[right as usize];
				loop {
					i += 1;
					while compare(&arr[i as usize], &*v) == Ordering::Less {
						i += 1
					}
					j -= 1;
					while compare(&*v, &arr[j as usize]) == Ordering::Less {
						if j == left {
							break;
						}
						j -= 1;
					}
					if i >= j {
						break;
					}
					arr.swap(i as usize, j as usize);
					if compare(&arr[i as usize], &*v) == Ordering::Equal {
						p += 1;
						arr.swap(p as usize, i as usize)
					}
					if compare(&*v, &arr[j as usize]) == Ordering::Equal {
						q -= 1;
						arr.swap(j as usize, q as usize)
					}
				}
			}

			arr.swap(i as usize, right as usize);
			j = i - 1;
			i += 1;
			let mut k: isize = left;
			while k < p {
				arr.swap(k as usize, j as usize);
				k += 1;
				j -= 1;
				assert!(k < arr.len() as isize);
			}
			k = right - 1;
			while k > q {
				arr.swap(i as usize, k as usize);
				k -= 1;
				i += 1;
				assert!(k != 0);
			}

			quicksort_helper(arr, left, j, compare);
			quicksort_helper(arr, i, right, compare);
		}

		fn quicksort_by<T, F>(arr: &mut [T], compare: F)
		where
			F: Fn(&T, &T) -> Ordering,
		{
			if arr.len() <= 1 {
				return;
			}

			let len = arr.len();
			quicksort_helper(arr, 0, (len - 1) as isize, &compare);
		}

		quicksort_by(&mut array, |a, b| a.cmp(b));

		array
	}

	fn heap(&self) -> [i32; SIZE] {
		let mut array = self.array.to_owned();

		fn sort(arr: &mut [i32; SIZE]) {
			let end = arr.len();
			for start in (0..end / 2).rev() {
				sift_down(arr, start, end - 1);
			}
			for end in (1..arr.len()).rev() {
				arr.swap(end, 0);
				sift_down(arr, 0, end - 1);
			}
		}
		fn sift_down(arr: &mut [i32; SIZE], start: usize, end: usize) {
			let mut root = start;
			loop {
				let mut child = root * 2 + 1; // Get the left child
				if child > end {
					break;
				}
				if child < end && arr[child] < arr[child + 1] {
					// Right child exists and is greater.
					child += 1;
				}

				if arr[root] < arr[child] {
					// If child is greater than root, swap'em!
					arr.swap(root, child);
					root = child;
				} else {
					break;
				}
			}
		}
		sort(&mut array);
		array
	}

	fn show_result(&self, f: &dyn Fn(&Self) -> [i32; SIZE]) {
		static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
		let start = Instant::now();
		f(&self);
		println!("{} Done in {}ms", SPARKLE, start.elapsed().as_millis());
	}
}

fn main() {
	let selections = &[
		"Exit!",
		"Bubble sort",
		"Selection sort",
		"Merge sort",
		"Heap sort",
		"Radix sort",
		"Quick sort",
	];

	let alg = Algs::new();

	loop {
		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("Pick a sorting alg")
			.default(0)
			.items(&selections[..])
			.interact()
			.unwrap();

		match selections[selection] {
			"Exit!" => {
				println!("GoodBye!!!");
				std::process::exit(0)
			}
			"Bubble sort" => alg.show_result(&Algs::bubble),
			"Selection sort" => alg.show_result(&Algs::select),
			"Merge sort" => alg.show_result(&Algs::merge_sort),
			"Heap sort" => alg.show_result(&Algs::heap),
			"Radix sort" => alg.show_result(&Algs::radix),
			"Quick sort" => alg.show_result(&Algs::quick),
			_ => todo!(),
		}
	}
}
