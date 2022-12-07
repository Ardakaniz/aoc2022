use std::{collections::HashMap, path::Path, path::PathBuf};

use crate::day::Day;

pub struct D07;

impl Day for D07 {
	type Return = usize;

	fn id() -> usize {
		07
	}

	fn lvl1(input: &str) -> usize {
		let mut dirs = D07::parse(input);
		D07::compute_total_size(Path::new("/"), &mut dirs);

		//println!("{:#?}", dirs);

		let sum = dirs
			.values()
			.filter(|v| v.total_size <= 100000)
			.fold(0, |acc, v| acc + v.total_size);

		sum
	}

	fn lvl2(input: &str) -> usize {
		let mut dirs = D07::parse(input);
		D07::compute_total_size(Path::new("/"), &mut dirs);

		//println!("{:#?}", dirs);

		let root_size = dirs[&PathBuf::from("/")].total_size;

		let space_used = 30000000 + root_size;
		let space_to_free = space_used - 70000000;

		let mut min_space = usize::MAX;
		for v in dirs.values() {
			let total_size = v.total_size;

			if total_size >= space_to_free && total_size < min_space {
				min_space = total_size;
			}
		}

		min_space
	}
}

impl D07 {
	fn parse(input: &str) -> HashMap<PathBuf, Dir> {
		let lines = input.lines().map(D07::tok_line);

		let mut directories: HashMap<PathBuf, Dir> = HashMap::new();
		let mut curr_dir = PathBuf::new();

		for line in lines {
			if let Line::Command(cmd) = line {
				match cmd {
					Cmd::ListDir => continue, // Expect stdout
					Cmd::ChangeDir(dir) => match dir {
						RelativePath::Root => curr_dir = PathBuf::from("/"),
						RelativePath::Parent => {
							curr_dir.pop();
						}
						RelativePath::Child(path) => {
							curr_dir.push(path);
						}
					},
				}
			} else if let Line::Stdout(out) = line {
				match out {
					Out::Dir(name) => {
						directories
							.entry(curr_dir.clone())
							.or_insert(Dir::default())
							.subdirs
							.push(curr_dir.join(name));
					}
					Out::File(size, _) => {
						directories
							.entry(curr_dir.clone())
							.or_insert(Dir::default())
							.files_size += size;
					}
				}
			}
		}

		directories
	}

	fn tok_line(line: &str) -> Line {
		let mut words = line.split(' ');

		let first = words.next().unwrap();
		let next = words.next().unwrap();

		match first {
			"$" => Line::Command(if next == "ls" {
				Cmd::ListDir
			} else {
				let where_ = words.next().unwrap();
				Cmd::ChangeDir(where_.into())
			}),
			"dir" => Line::Stdout(Out::Dir(next)),
			size => {
				let size = size.parse().unwrap();
				let name = next;

				Line::Stdout(Out::File(size, name))
			}
		}
	}

	fn compute_total_size(path: &Path, dirs: &mut HashMap<PathBuf, Dir>) {
		dirs.get_mut(path).unwrap().total_size += dirs[path].files_size;

		for subdir in dirs[path].subdirs.clone() {
			D07::compute_total_size(&subdir, dirs);

			dirs.get_mut(path).unwrap().total_size += dirs[&subdir].total_size;
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line<'a> {
	Command(Cmd<'a>),
	Stdout(Out<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd<'a> {
	ChangeDir(RelativePath<'a>),
	ListDir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Out<'a> {
	Dir(&'a str),         // Name
	File(usize, &'a str), // Size, Name
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RelativePath<'a> {
	Root,
	Parent,
	Child(&'a str),
}

impl<'a> From<&'a str> for RelativePath<'a> {
	fn from(s: &'a str) -> Self {
		match s {
			"/" => Self::Root,
			".." => Self::Parent,
			child => Self::Child(child),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Dir {
	files_size: usize,
	subdirs: Vec<PathBuf>,
	total_size: usize,
}
