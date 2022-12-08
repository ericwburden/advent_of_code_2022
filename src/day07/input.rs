use crate::day07::Input;
use anyhow::{anyhow, bail, Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Represents a directory in the file tree, including the directory label,
/// a list of the directory's contents, and the total size of all the files
/// contained in the directory and all sub-directories.
#[derive(Debug, Clone)]
pub struct Dir<'a> {
    pub label: &'a str,
    pub contents: Vec<FileSystemObj<'a>>,
    pub size: u32,
}

impl<'a> Dir<'a> {
    /// Create an empty directory with the given label
    fn from(label: &'a str) -> Self {
        let contents = Vec::new();
        let size = 0;
        Dir {
            label,
            contents,
            size,
        }
    }
}

/// Represents a file in the file tree, including its name and file size
#[derive(Debug, Clone)]
pub struct File<'a> {
    pub label: &'a str,
    pub size: u32,
}

/// Enum to allow both types of items to be stored in a single vector. Both types
/// are in Rc<RefCell<T>> wrappers to TODO
#[derive(Debug, Clone)]
pub enum FileSystemObj<'a> {
    Dir(Rc<RefCell<Dir<'a>>>),
    File(Rc<RefCell<File<'a>>>),
}

impl<'a> FileSystemObj<'a> {
    /// Create a wrapped Dir representing the root folder of the file system
    fn root() -> Self {
        let dir = Dir::from("/");
        FileSystemObj::Dir(Rc::new(RefCell::new(dir)))
    }

    /// Return the label applied to a directory or file
    fn label(&self) -> &str {
        match self {
            FileSystemObj::Dir(d) => d.borrow().label,
            FileSystemObj::File(f) => f.borrow().label,
        }
    }

    /// Search the contents of a file system object and return the child object
    /// indicated by `label`.
    fn get_child(&self, label: &str) -> Option<FileSystemObj<'a>> {
        match self {
            // Files don't have children
            FileSystemObj::File(_) => None,

            // Just iterate through the contents of the directory and return
            // the first item whose label matches the given label.Return None
            // if the directory doesn't contain any item with the given label.
            FileSystemObj::Dir(dir) => dir
                .borrow()
                .contents
                .iter()
                .find(|c| c.label() == label)
                .cloned(),
        }
    }

    /// Add items to the contents of this object
    fn add_contents<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = FileSystemObj<'a>>,
    {
        match self {
            FileSystemObj::File(_) => (),
            FileSystemObj::Dir(dir) => {
                dir.borrow_mut().contents.extend(iter);
            }
        }
    }
}

/// Represents a command from the input file. Commands come in one of four flavors.
#[derive(Debug, Clone)]
pub enum Cmd<'a> {
    MoveIn(Dir<'a>),
    MoveUp,
    MoveRoot,
    List(Vec<FileSystemObj<'a>>),
}

/// Module to wrap the parsers needed to parse the input file into commands
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{alpha1, space1, u32},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        Finish, IResult,
    };

    /// Nom parser for "dir bacon" -> Rc<RefCell<Dir { label: "bacon" }>>
    fn dir(s: &str) -> IResult<&str, Rc<RefCell<Dir>>> {
        let (s, label) = preceded(tag("dir "), alpha1)(s)?;
        let dir = Dir::from(label);
        Ok((s, Rc::new(RefCell::new(dir))))
    }

    /// Nom parser for "123 eggs.txt" -> Rc<RefCell<File { size: 123, label: "eggs.txt" }>> 
    fn file(s: &str) -> IResult<&str, Rc<RefCell<File>>> {
        let (s, (size, label)) = separated_pair(u32, space1, take_until("\n"))(s)?;
        let file = File { size, label };
        Ok((s, Rc::new(RefCell::new(file))))
    }

    /// Nom parser for parsing one of the file objects listed from an `ls` command
    fn fs_obj(s: &str) -> IResult<&str, FileSystemObj> {
        alt((map(dir, FileSystemObj::Dir), map(file, FileSystemObj::File)))(s)
    }

    /// Nom parser for a list of newline separated results from an `ls` command 
    fn contents(s: &str) -> IResult<&str, Vec<FileSystemObj>> {
        separated_list1(tag("\n"), fs_obj)(s)
    }

    /// Nom parser for the various `cd` commands
    fn cd_cmd(s: &str) -> IResult<&str, Cmd> {
        let (s, cmd_str) = preceded(tag("$ cd "), take_until("\n"))(s)?;
        let cmd = match cmd_str {
            "/" => Cmd::MoveRoot,
            ".." => Cmd::MoveUp,
            _ => Cmd::MoveIn(Dir::from(cmd_str)),
        };
        Ok((s, cmd))
    }

    /// Nom parser for the `ls` command. Grabs the command line and the lines
    /// that follow listing the files and directories.
    fn ls_cmd(s: &str) -> IResult<&str, Cmd> {
        let (s, listed) = preceded(tag("$ ls\n"), contents)(s)?;
        Ok((s, Cmd::List(listed)))
    }

    /// Nom parser for either a `cd` or `ls` command
    fn command(s: &str) -> IResult<&str, Cmd> {
        alt((cd_cmd, ls_cmd))(s)
    }

    /// Nom parser to parse all commands from the input into a list of Cmd
    pub fn commands(s: &str) -> Result<Vec<Cmd>> {
        let result = separated_list1(tag("\n"), command)(s).finish();
        let (_, cmds) = result.map_err(|_| anyhow!("Could not parse commands!"))?;
        Ok(cmds)
    }
}

/// Represents the entire filesystem, which is a linked tree of all the filesystem
/// objects. Contains the root node.
#[derive(Debug, Clone)]
pub struct FileSystem<'a>(pub FileSystemObj<'a>);

impl<'a> TryFrom<Vec<Cmd<'a>>> for FileSystem<'a> {
    type Error = Error;

    fn try_from(commands: Vec<Cmd<'a>>) -> Result<Self, Self::Error> {
        let root = FileSystemObj::root();
        let mut open_dirs = vec![root.clone()];

        for command in commands {
            // This is safe, there's nothing in the loop that would cause us to
            // empty `open_dirs` that isn't already handled.
            let current_dir = open_dirs.last_mut().unwrap();

            // Based on the command we're looking at...
            match command {
                // Move into a new directory by getting that directory's reference
                // from the current directory's contents and pushing that reference
                // to the end of the list of open directories.
                Cmd::MoveIn(dir) => {
                    let dir = current_dir.get_child(dir.label).unwrap();
                    open_dirs.push(dir);
                }

                // Move up out of the current directory by dropping the last directory
                // from the list of open directories.
                Cmd::MoveUp => {
                    open_dirs
                        .pop()
                        .ok_or(anyhow!("Cannot 'cd ..' out of root!"))?;
                }

                // Move to the root directory by dropping all but the first (root)
                // directory from the list of open directories.
                Cmd::MoveRoot => open_dirs.truncate(1),

                // Process a command to list contents by adding all the files and
                // directories listed as children of the currently open directory.
                Cmd::List(mut objs) => current_dir.add_contents(objs.drain(..)),
            }
        }
        Ok(FileSystem(root))
    }
}

impl FileSystem<'_> {
    /// Fill in the sizes of all the directories in the file system by recursively
    /// walking the file system.
    fn calculate_directory_sizes(&self) {
        fn size(obj: FileSystemObj) -> u32 {
            match obj {
                // Base case, return the size of a file
                FileSystemObj::File(file) => file.borrow().size,

                // Otherwise, recursively calculate the total size of all files
                // contained in this directory and its subdirectories and return
                // that total.
                FileSystemObj::Dir(dir) => {
                    let mut total = 0;
                    for child in dir.borrow().contents.iter() {
                        total += size(child.clone());
                    }
                    dir.borrow_mut().size = total;
                    total
                }
            }
        }

        // Perform the walk
        size(self.0.clone());
    }
}

const INPUT: &str = include_str!("../../input/07/input.txt");

/// Read the input by first parsing all the commands from the input file, then
/// following those commands to build up a tree structure for the file system, 
/// finally filling in all the directory sizes and returning the file system struct.
pub fn read<'a>() -> Input<'a> {
    let commands = parser::commands(INPUT).expect("Could not parse input!");
    let mut fs = FileSystem::try_from(commands).expect("Could not build filesystem!");
    fs.calculate_directory_sizes();
    fs
}

#[cfg(test)]
mod test {
    use super::*;

    impl<'a> Display for Cmd<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Cmd::MoveIn(d) => write!(f, "$ cd {}", d.label),
                Cmd::MoveUp => write!(f, "$ cd .."),
                Cmd::MoveRoot => write!(f, "$ cd /"),
                Cmd::List(objs) => {
                    write!(f, "$ ls")?;
                    for obj in objs {
                        writeln!(f)?;
                        match obj {
                            FileSystemObj::Dir(dir) => write!(f, "dir {}", dir.borrow().label)?,
                            FileSystemObj::File(file) => {
                                write!(f, "{} {}", file.borrow().size, file.borrow().label)?
                            }
                        }
                    }
                    write!(f, "")
                }
            }
        }
    }
    #[test]
    fn name() {
        todo!()
    }
}
