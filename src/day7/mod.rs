use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;
use crate::day::{Answer, Day};


pub struct Day7 {
    current_directory: DirectoryPtr,
    command_parse_re: Regex,
    filesystem_head: DirectoryPtr,
    directories: Vec<DirectoryPtr>,
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
                    let directory = Rc::new(RefCell::new(
                        Directory::new(directory_name, Some(Rc::clone(&self.current_directory)))
                    ));
                    self.current_directory.borrow_mut().subdirectories.push(Rc::clone(&directory));
                    self.directories.push(Rc::clone(&directory));
                } else {    // file
                    let captures = ls_output_file_re.captures(line).unwrap();
                    let filename = captures.name("name").unwrap().as_str().to_string();
                    let file_size = captures.name("size").unwrap().as_str().parse::<u32>().unwrap();
                    self.current_directory.borrow_mut().contents.push(File {
                        name: filename,
                        size: file_size,
                    })
                }
            }
        }

        let size_required = self.filesystem_head.borrow().calculate_size() - 40000000;
        let mut part_1 = 0u32;
        let mut part_2 = 999999999u32;
        for directory in &self.directories {
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
        let starting_directory = Rc::new(RefCell::new(
            Directory::new("/", None)
        ));
        Day7 {
            current_directory: Rc::clone(&starting_directory),
            command_parse_re: Regex::new(r"^\$ (?P<command>\w+) ?(?P<argument>[\w/.]*)").unwrap(),
            filesystem_head: Rc::clone(&starting_directory),
            directories: vec![Rc::clone(&starting_directory)],
        }
    }
    fn parse_command(&mut self, command: &str) {
        let parts = self.command_parse_re.captures(command).unwrap();
        let command = parts.name("command").unwrap().as_str();
        if command == "cd" {
            let argument = parts.name("argument").unwrap().as_str();
            if argument == ".." {
                //self.current_directory = self.current_directory.borrow().get_parent().unwrap();
                let parent = self.current_directory.borrow().get_parent().unwrap();
                self.current_directory = parent;
            } else if argument == "/" {
                self.current_directory = Rc::clone(&self.filesystem_head);
            } else {
                let subdirectory = self.current_directory.borrow()
                    .get_subdirectory(argument).unwrap();
                self.current_directory = subdirectory;
            }
        } else if command == "ls" {
            // do nothing - the parser in Day7::run() will handle this input
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