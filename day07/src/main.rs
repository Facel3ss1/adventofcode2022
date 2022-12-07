use std::collections::HashMap;

#[derive(Debug)]
struct FileSystem {
    root_directory: Directory,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root_directory: Directory::new(),
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

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

#[derive(Debug)]
struct FileIndex(usize);
#[derive(Debug, Clone, Copy)]
struct DirectoryIndex(usize);

#[derive(Debug)]
struct Directory {
    directories: HashMap<String, DirectoryIndex>,
    files: Vec<FileIndex>,
}

impl Directory {
    fn new() -> Self {
        Self {
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
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
    File(usize, String),
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
            let name = parts.next().unwrap().to_string();

            Line::File(size, name)
        };

        lines.push(line);
    }

    lines
}

fn parse_commands(lines: Vec<Line>) -> FileSystem {
    let mut filesystem = FileSystem::new();
    let mut directory_stack: Vec<DirectoryIndex> = Vec::new();
    let mut current_directory: Option<DirectoryIndex> = None;

    for line in lines {
        match line {
            Line::Cd(arg) => {
                match arg {
                    CdArg::Parent => {
                        directory_stack.pop();
                    }
                    CdArg::Named(dir) => {
                        let directory_index = filesystem
                            .get_directory(current_directory)
                            .directories
                            .get(&dir)
                            .expect("directory not found");
                        directory_stack.push(*directory_index);
                    }
                }
                current_directory = directory_stack.last().copied();
            }
            Line::File(size, name) => {
                let file_index = FileIndex(filesystem.files.len());
                filesystem.files.push(File { size, name });
                filesystem
                    .get_directory_mut(current_directory)
                    .files
                    .push(file_index);
            }
            Line::Directory(name) => {
                let directory_index = DirectoryIndex(filesystem.directories.len());
                filesystem.directories.push(Directory::new());
                filesystem
                    .get_directory_mut(current_directory)
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
    part1_sum: &mut usize,
) -> usize {
    let mut sum = 0;
    let current_directory = filesystem.get_directory(directory_index);

    for file_index in current_directory.files.iter() {
        sum += filesystem.files[file_index.0].size;
    }

    for directory_index in current_directory.directories.values().copied() {
        sum += traverse_directories(filesystem, Some(directory_index), part1_sum);
    }

    if sum <= 100_000 {
        *part1_sum += sum;
    }

    sum
}

fn part1(filesystem: &FileSystem) -> usize {
    let mut sum = 0;
    traverse_directories(filesystem, None, &mut sum);
    sum
}

fn main() {
    let lines = parse_lines(include_str!("input.txt"));
    let filesystem = parse_commands(lines);

    println!("Part 1: {}", part1(&filesystem));
}
