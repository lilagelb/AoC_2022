use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;
use crate::day::{Answer, Day};


pub struct Day7 {
    filesystem: Filesystem,
    command_parse_re: Regex,
}
impl Day for Day7{
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(7);

        let ls_output_dir_re = Regex::new(r"dir (?P<name>\w+)").unwrap();
        let ls_output_file_re = Regex::new(r"(?P<size>\d+) (?P<name>.+)").unwrap();

        for line in &input {
            if line.starts_with("$") {
                self.parse_command(line);
            } else {
                if line.starts_with("d") {  // directory
                    let captures = ls_output_dir_re.captures(line).unwrap();
                    let directory_name = captures.name("name").unwrap().as_str();
                    self.filesystem.add_directory(directory_name.to_string());
                } else {    // file
                    let captures = ls_output_file_re.captures(line).unwrap();
                    let filename = captures.name("name").unwrap().as_str().to_string();
                    let file_size = captures.name("size").unwrap().as_str().parse::<u32>().unwrap();
                    self.filesystem.add_file(filename, file_size);
                }
            }
        }

        let size_required = self.filesystem.head.borrow().calculate_size() - 40000000;
        let mut part_1 = 0u32;
        let mut part_2 = 999999999u32;
        for directory in &self.filesystem.directories {
            let size = directory.borrow().calculate_size();
            if size <= 100000 {
                part_1 += size;
            }
            if size >= size_required && size < part_2 {
                part_2 = size;
            }
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            filesystem: Filesystem::new(),
            command_parse_re: Regex::new(r"^\$ (?P<command>\w+) ?(?P<argument>[\w/.]*)").unwrap(),
        }
    }
    fn parse_command(&mut self, command: &str) {
        let parts = self.command_parse_re.captures(command).unwrap();
        let command = parts.name("command").unwrap().as_str();
        if command == "cd" {
            let argument = parts.name("argument").unwrap().as_str();
            self.filesystem.change_directory(argument);
        } else if command == "ls" {
            // do nothing - the parser in Day7::run() will handle this input
        }
    }
}

struct Filesystem {
    head: DirectoryPtr,
    current_directory: DirectoryPtr,
    directories: Vec<DirectoryPtr>,
}
impl Filesystem {
    fn new() -> Filesystem {
        let head = Rc::new(RefCell::new(Directory {
            name: "/".to_string(),
            parent: None,
            subdirectories: Vec::new(),
            contents: Vec::new(),
        }));
        Filesystem {
            head: Rc::clone(&head),
            current_directory: Rc::clone(&head),
            directories: vec![Rc::clone(&head)],
        }
    }

    fn add_directory(&mut self, name: String) {
        let new_directory = Rc::new(RefCell::new(Directory {
            name,
            parent: Some(Rc::clone(&self.current_directory)),
            subdirectories: Vec::new(),
            contents: Vec::new(),
        }));
        self.current_directory.borrow_mut().subdirectories.push(Rc::clone(&new_directory));
        self.directories.push(Rc::clone(&new_directory));
    }

    fn add_file(&mut self, name: String, size: u32) {
        let file = File {
            name, size
        };
        self.current_directory.borrow_mut().contents.push(file);
    }

    fn change_directory(&mut self, directory: &str) {
        if directory == ".." {
            let parent = self.current_directory.borrow().get_parent().unwrap();
            self.current_directory = parent;
        } else if directory == "/" {
            self.current_directory = Rc::clone(&self.head);
        } else {
            let subdirectory = self.current_directory.borrow()
                .get_subdirectory(directory).unwrap();
            self.current_directory = subdirectory;
        }
    }
}

struct File {
    name: String,
    size: u32,
}
struct Directory {
    name: String,
    parent: Option<DirectoryPtr>,
    subdirectories: Vec<DirectoryPtr>,
    contents: Vec<File>,
}
impl Directory {
    fn new(name: &str, parent: Option<DirectoryPtr>) -> Directory {
        Directory {
            name: name.to_string(),
            parent,
            subdirectories: Vec::new(),
            contents: Vec::new(),
        }
    }
    fn get_parent(&self) -> Option<DirectoryPtr> {
        match &self.parent {
            Some(directory) => Some(Rc::clone(directory)),
            None => None,
        }
    }
    fn get_subdirectory(&self, subdirectory: &str) -> Option<DirectoryPtr> {
        for directory in &self.subdirectories {
            if directory.borrow().name == subdirectory {
                return Some(Rc::clone(directory));
            }
        }
        return None;
    }
    fn calculate_size(&self) -> u32 {
        let total_file_size = self.contents.iter()
            .map(|file| file.size)
            .sum::<u32>();
        let total_subdirectory_size = self.subdirectories.iter()
            .map(|subdirectory| subdirectory.borrow().calculate_size())
            .sum::<u32>();
        total_file_size + total_subdirectory_size
    }
}

type DirectoryPtr = Rc<RefCell<Directory>>;