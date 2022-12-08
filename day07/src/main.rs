use std::collections::HashMap;

#[derive(Debug, Default)]
struct FileSystem {
    root_directory: Directory,
    directories: Vec<Directory>,
}

impl FileSystem {
    fn get_directory(&self, directory_index: Option<DirectoryIndex>) -> &Directory {
        match directory_index {
            None => &self.root_directory,
            Some(directory_index) => &self.directories[directory_index.0],
        }
    }

    fn get_directory_mut(&mut self, directory_index: Option<DirectoryIndex>) -> &mut Directory {
        match directory_index {
            None => &mut self.root_directory,
            Some(directory_index) => &mut self.directories[directory_index.0],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DirectoryIndex(usize);

#[derive(Debug, Default)]
struct Directory {
    directories: HashMap<String, DirectoryIndex>,
    size: usize,
}

#[derive(Debug)]
enum CdArg {
    Parent,
    Named(String),
}

#[derive(Debug)]
enum Line {
    Cd(CdArg),
    Directory(String),
    File(usize),
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input.lines().skip(2) {
        let mut parts = line.split_whitespace();

        let first_part = parts.next().unwrap();
        let line = if first_part == "$" {
            let command = parts.next().unwrap();
            if command != "cd" {
                continue;
            }

            let arg_str = parts.next().unwrap();
            let arg = if arg_str == ".." {
                CdArg::Parent
            } else {
                CdArg::Named(arg_str.to_string())
            };

            Line::Cd(arg)
        } else if first_part == "dir" {
            Line::Directory(parts.next().unwrap().to_string())
        } else {
            let size = first_part.parse().unwrap();
            Line::File(size)
        };

        lines.push(line);
    }

    lines
}

fn parse_commands(lines: Vec<Line>) -> FileSystem {
    let mut filesystem = FileSystem::default();
    let mut directory_stack: Vec<DirectoryIndex> = Vec::new();

    for line in lines {
        match line {
            Line::Cd(arg) => match arg {
                CdArg::Parent => {
                    directory_stack.pop();
                }
                CdArg::Named(dir) => {
                    let directory_index = filesystem
                        .get_directory(directory_stack.last().copied())
                        .directories
                        .get(&dir)
                        .expect("directory not found");
                    directory_stack.push(*directory_index);
                }
            },
            Line::File(size) => {
                filesystem.get_directory_mut(None).size += size;

                for directory_index in directory_stack.iter().copied() {
                    filesystem.get_directory_mut(Some(directory_index)).size += size
                }
            }
            Line::Directory(name) => {
                let directory_index = DirectoryIndex(filesystem.directories.len());
                filesystem.directories.push(Directory::default());
                filesystem
                    .get_directory_mut(directory_stack.last().copied())
                    .directories
                    .insert(name, directory_index);
            }
        }
    }

    filesystem
}

fn traverse_directories(
    filesystem: &FileSystem,
    directory_index: Option<DirectoryIndex>,
    used_space: usize,
    part1_sum: &mut usize,
    part2_size: &mut usize,
) {
    let current_directory = filesystem.get_directory(directory_index);
    let directory_size = current_directory.size;

    if directory_size <= 100_000 {
        *part1_sum += directory_size;
    }

    if (used_space - directory_size) + 30_000_000 <= 70_000_000 && directory_size < *part2_size {
        *part2_size = directory_size;
    }

    for index in current_directory.directories.values().copied() {
        traverse_directories(filesystem, Some(index), used_space, part1_sum, part2_size);
    }
}

fn solve(filesystem: &FileSystem) -> (usize, usize) {
    let used_space = filesystem.get_directory(None).size;

    let mut part1_sum = 0;
    let mut part2_size = used_space;

    traverse_directories(
        filesystem,
        None,
        used_space,
        &mut part1_sum,
        &mut part2_size,
    );

    (part1_sum, part2_size)
}

fn main() {
    let lines = parse_lines(include_str!("input.txt"));
    let filesystem = parse_commands(lines);

    let (part1, part2) = solve(&filesystem);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
