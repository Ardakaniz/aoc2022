use crate::day::Day;

pub struct D10;

impl Day for D10 {
	type Return = i32;
	type Return2 = String;

	fn id() -> usize {
		10
	}

	fn lvl1(input: &str) -> Self::Return {
		let prg = Program::new(input);

		prg
			.iter_reg()
			.enumerate()
			.map(|(i, x)| (i + 1) as i32 * x)
			.skip(19)
			.step_by(40)
			.sum()
	}

	fn lvl2(input: &str) -> Self::Return2 {
		const WIDTH: usize = 40;
		const _HEIGHT: usize = 6;

		let prg = Program::new(input);

		String::from("\n")
			+ &prg
				.iter_reg()
				.enumerate()
				.map(|(i, x)| {
					let i = i % WIDTH;
					let append_newline = i == WIDTH - 1;
					let i = i as i32;

					if x - 1 <= i && i <= x + 1 {
						if append_newline {
							"#\n"
						} else {
							"#"
						}
					} else {
						if append_newline {
							".\n"
						} else {
							"."
						}
					}
				})
				.collect::<String>()
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instr {
	Nop,
	Add(i32),
}

impl std::str::FromStr for Instr {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(' ');

		match split.next().unwrap() {
			"noop" => Ok(Self::Nop),
			"addx" => split
				.next()
				.ok_or_else(|| "Invallid addx instr".into())
				.and_then(|s| {
					s.parse()
						.map_err(|e: std::num::ParseIntError| e.to_string())
				})
				.map(Self::Add),
			instr => Err(format!("Unknown instr {}", instr)),
		}
	}
}

struct Program<'a>(&'a str);

impl<'a> Program<'a> {
	fn new(prg: &'a str) -> Self {
		Self(prg)
	}

	fn iter_execs(&self) -> impl Iterator<Item = Exec> + '_ {
		ProgramIter::new(self)
	}

	fn iter_reg(&self) -> impl Iterator<Item = i32> + '_ {
		ProgramIter::new(self).scan(1, |x_reg, e| {
			let x_backup = *x_reg;

			*x_reg += match e {
				Exec::Pending => 0,
				Exec::Instr(i) => match i {
					Instr::Nop => 0,
					Instr::Add(v) => v,
				},
			};

			Some(x_backup)
		})
	}
}

#[derive(Debug, PartialEq, Eq)]
enum Exec {
	Instr(Instr),
	Pending,
}

struct ProgramIter<'a> {
	iter: Box<dyn Iterator<Item = Instr> + 'a>,
	pending_instr: Option<Instr>,
	pending_counter: u32,
}

impl<'a> ProgramIter<'a> {
	fn new(prg: &'a Program) -> Self {
		Self {
			iter: Box::new(prg.0.lines().map(|l| l.parse().unwrap())),
			pending_instr: None,
			pending_counter: 0,
		}
	}
}

impl<'a> Iterator for ProgramIter<'a> {
	type Item = Exec;

	fn next(&mut self) -> Option<Self::Item> {
		if self.pending_counter > 1 {
			self.pending_counter -= 1;
			Some(Exec::Pending)
		} else if let Some(i) = self.pending_instr.take() {
			Some(Exec::Instr(i))
		} else {
			match self.iter.next()? {
				Instr::Add(v) => {
					self.pending_instr = Some(Instr::Add(v));
					self.pending_counter = 2;

					self.next()
				}
				i => Some(Exec::Instr(i)),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parsing() {
		let input = "
noop
addx 3
addx -5
"
		.trim();

		let instr: Vec<Instr> = input.lines().map(|l| l.parse().unwrap()).collect();
		assert_eq!(instr, [Instr::Nop, Instr::Add(3), Instr::Add(-5)])
	}

	#[test]
	fn test_prg() {
		use self::Instr::*;
		use Exec::*;

		let input = "
noop
addx 3
addx -5
"
		.trim();

		let prg: Vec<_> = Program::new(input).iter_execs().collect();

		assert_eq!(
			prg,
			[Instr(Nop), Pending, Instr(Add(3)), Pending, Instr(Add(-5))]
		);
	}

	#[test]
	fn test_lvl1() {
		let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
		.trim();

		assert_eq!(D10::lvl1(input), 13140);
	}

	#[test]
	fn test_lvl2() {
		let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
		.trim();

		let expected = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

		assert_eq!(D10::lvl2(input), expected);
	}
}
