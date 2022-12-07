pub trait Day {
	type Return: std::fmt::Display;

	fn id() -> usize;

	fn lvl1(input: &str) -> Self::Return;
	fn lvl2(input: &str) -> Self::Return;
}
