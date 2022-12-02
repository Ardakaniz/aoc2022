use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn read_file() -> std::io::Result<String> {
	std::fs::read_to_string("inputs/01")
}

pub fn lvl1() -> std::io::Result<u32> {
	let input = read_file()?;

	let elves = input.split("\n\n");

	Ok(
		elves
			.map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
			.max()
			.unwrap(),
	)
}

pub fn lvl2() -> std::io::Result<u32> {
	let input = read_file()?;

	let elves = input.split("\n\n");
	let calories_it = elves.map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>());

	let mut heap = BinaryHeap::with_capacity(3);
	for calories in calories_it {
		heap.push(Reverse(calories));
		if heap.len() > 3 {
			heap.pop();
		}
	}

	Ok(heap.iter().map(|r| r.0).sum())
}
