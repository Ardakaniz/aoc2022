mod day;

//mod d01;
//mod d02;
//mod d06;
mod d07;

fn read_input(day: usize) -> std::io::Result<String> {
	std::fs::read_to_string(format!("inputs/{:0>2}", day))
}

fn exec<T: day::Day>() {
	let id = T::id();

	let input = read_input(id).unwrap();
	println!("\nDay {}: Lvl1: {}", T::id(), T::lvl1(&input));
	println!("Day {}: Lvl2: {}", T::id(), T::lvl2(&input));
}

fn main() {
	//println!("Day 1: Lvl1: {}", d01::lvl1()?);
	//println!("Day 1: Lvl2: {}", d01::lvl2()?);

	exec::<d07::D07>();
}
