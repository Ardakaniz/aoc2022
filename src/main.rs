mod d01;

fn main() -> std::io::Result<()> {
	println!("Day 1: Lvl1: {}", d01::lvl1()?);
	println!("Day 1: Lvl2: {}", d01::lvl2()?);

	Ok(())
}
