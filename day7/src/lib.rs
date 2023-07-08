use std::{cell::RefCell, fmt::Display, rc::Rc, str::Lines, sync::atomic::AtomicUsize};

use itertools::Itertools;

struct CLIParser<'a, 'b> {
    input: &'a mut Lines<'b>,
}

impl<'a, 'b> CLIParser<'a, 'b> {
    fn new(input: &'a mut Lines<'b>) -> Self {
        Self { input }
    }

    fn parse(&mut self) -> Rc<RefCell<Filesystem>> {
        use Filesystem::*;
        let fs = Command::parse_commands(&mut self.input).iter().fold(
            Rc::new(RefCell::new(Directory {
                name: Filesystem::ROOT_NAME,
                children: Vec::new(),
                parent: None,
            })),
            |pwd, command| command.deduce_fs(pwd),
        );
        let b = fs.borrow().root();
        if let Some(root) = b {
            root
        } else {
            fs
        }
    }
}

enum Command<'a> {
    CD { path: &'a str },
    LS { output: Vec<LsOutput<'a>> },
}

enum LsOutput<'a> {
    Dir { name: &'a str },
    File { size: usize, name: &'a str },
}

impl<'a> Command<'a> {
    const COMMAND_LINE_START: char = '$';

    fn is_command(input: &str) -> bool {
        input.starts_with(Self::COMMAND_LINE_START)
    }

    fn parse_commands(lines: &mut Lines<'a>) -> Vec<Self> {
        let mut v = Vec::new();
        while let Some(command_line) = lines.next() {
            let res = lines.take_while_ref(|line| !Self::is_command(line)).fold(
                vec![command_line],
                |mut acc, cur| {
                    acc.push(cur);
                    acc
                },
            );
            v.push(res);
        }

        v.into_iter()
            .map(|command_group| {
                use Command::*;
                let s = command_group[0].split_ascii_whitespace().collect_vec();
                match s[1] {
                    "cd" => CD { path: s[2] },
                    "ls" => {
                        let output = command_group[1..]
                            .iter()
                            .map(|o| {
                                use LsOutput::*;
                                let v = o.split_ascii_whitespace().collect_vec();
                                match v[0] {
                                    "dir" => Dir { name: v[1] },
                                    _ => File {
                                        size: v[0].parse::<usize>().expect("safe"),
                                        name: v[1],
                                    },
                                }
                            })
                            .collect_vec();
                        LS { output }
                    }
                    _ => panic!("impossible"),
                }
            })
            .collect_vec()
    }

    fn deduce_fs(&self, pwd: Rc<RefCell<Filesystem<'a>>>) -> Rc<RefCell<Filesystem<'a>>> {
        use Command::*;
        match self {
            CD { path } => match *path {
                "/" => {
                    let b = pwd.borrow().root();
                    if let Some(root) = b {
                        root
                    } else {
                        pwd
                    }
                }
                ".." => {
                    let b = pwd.borrow().parent();
                    if let Some(parent) = b {
                        parent
                    } else {
                        pwd
                    }
                }
                dir => {
                    // Move into target or create it in our FS.
                    if let Some(child) = pwd.borrow().get_child(dir) {
                        child
                    } else {
                        let child = Rc::new(RefCell::new(Filesystem::Directory {
                            name: dir,
                            parent: Some(Rc::clone(&pwd)),
                            children: Vec::new(),
                        }));
                        pwd.borrow_mut().add_child(Rc::clone(&child)).expect("safe");
                        child
                    }
                }
            },
            LS { output } => {
                output.iter().for_each(|out| {
                    use LsOutput::*;
                    match out {
                        Dir { name } => {
                            if !pwd.borrow().has(name) {
                                let child = Rc::new(RefCell::new(Filesystem::Directory {
                                    name,
                                    parent: Some(Rc::clone(&pwd)),
                                    children: Vec::new(),
                                }));
                                pwd.borrow_mut().add_child(Rc::clone(&child)).expect("safe");
                            }
                        }
                        File { size, name } => {
                            if !pwd.borrow().has(name) {
                                let child = Rc::new(RefCell::new(Filesystem::File {
                                    name,
                                    size: *size,
                                    parent: Rc::clone(&pwd),
                                }));
                                pwd.borrow_mut().add_child(Rc::clone(&child)).expect("safe");
                            }
                        }
                    };
                });
                pwd
            }
        }
    }
}

enum Filesystem<'a> {
    Directory {
        name: &'a str,
        parent: Option<Rc<RefCell<Filesystem<'a>>>>, // Should only be Directory...
        children: Vec<Rc<RefCell<Filesystem<'a>>>>,
    },
    File {
        name: &'a str,
        size: usize,
        parent: Rc<RefCell<Filesystem<'a>>>, // Should only be Directory...
    },
}

impl<'a> Filesystem<'a> {
    const ROOT_NAME: &'static str = "/";

    fn is_dir(&self) -> bool {
        use Filesystem::*;
        match self {
            Directory { .. } => true,
            File { .. } => false,
        }
    }

    fn name(&self) -> &str {
        use Filesystem::*;
        match self {
            Directory { name, .. } => name,
            File { name, .. } => name,
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<Filesystem<'a>>>> {
        use Filesystem::*;
        match self {
            File { parent, .. } => Some(Rc::clone(parent)),
            Directory { parent, .. } => parent.clone(),
        }
    }

    fn root(&self) -> Option<Rc<RefCell<Filesystem<'a>>>> {
        use Filesystem::*;
        match self {
            Directory { parent: None, .. } => None,
            File { parent, .. }
            | Directory {
                parent: Some(parent),
                ..
            } => {
                if parent.borrow().name() == Self::ROOT_NAME {
                    Some(Rc::clone(parent))
                } else {
                    parent.borrow().root()
                }
            }
        }
    }

    fn has(&self, p: &str) -> bool {
        use Filesystem::*;
        match self {
            Directory { children, .. } => children
                .iter()
                .find(|child| child.as_ref().borrow().name() == p)
                .is_some(),
            File { .. } => false,
        }
    }

    fn get_child(&self, p: &str) -> Option<Rc<RefCell<Filesystem<'a>>>> {
        use Filesystem::*;
        match self {
            Directory { children, .. } => children
                .iter()
                .find(|child| child.as_ref().borrow().name() == p)
                .cloned(),
            File { .. } => None,
        }
    }

    fn size(&self) -> usize {
        use Filesystem::*;
        match self {
            File { size, .. } => *size,
            Directory { children, .. } => children.iter().map(|child| child.borrow().size()).sum(),
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<Filesystem<'a>>>) -> Result<(), String> {
        use Filesystem::*;
        match self {
            Directory { children, .. } => {
                children.push(Rc::clone(&child));
                Ok(())
            }
            File { .. } => Err("Not a directory".to_string()),
        }
    }
}

struct DepthFirstIteratorFS<'a> {
    waiting_stack: Vec<Rc<RefCell<Filesystem<'a>>>>,
}

impl<'a> From<Rc<RefCell<Filesystem<'a>>>> for DepthFirstIteratorFS<'a> {
    fn from(value: Rc<RefCell<Filesystem<'a>>>) -> Self {
        DepthFirstIteratorFS {
            waiting_stack: vec![value],
        }
    }
}

impl<'a> Iterator for DepthFirstIteratorFS<'a> {
    type Item = Rc<RefCell<Filesystem<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.waiting_stack.pop()?;
        use Filesystem::*;
        match &*node.borrow() {
            File { .. } => {}
            Directory { children, .. } => children.iter().rev().for_each(|child| {
                self.waiting_stack.push(Rc::clone(child));
            }),
        };
        Some(node)
    }
}

static DISPLAY_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<'a> Display for Filesystem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Filesystem::*;
        match self {
            File { name, size, .. } => write!(f, "- {name} (file, size={size})"),
            Directory { name, children, .. } => write!(f, "- {name} (dir)").and_then(|_| {
                DISPLAY_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let c = children
                    .iter()
                    .map(|child| {
                        let alignment = " "
                            .repeat(DISPLAY_COUNTER.load(std::sync::atomic::Ordering::Relaxed) * 4);
                        write!(f, "\n{alignment}{}", child.borrow())
                    })
                    .collect::<Result<(), _>>();
                DISPLAY_COUNTER.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                c
            }),
        }
    }
}

pub fn solve_part1(mut input: &mut Lines) -> usize {
    let mut parser = CLIParser::new(&mut input);
    let fs = parser.parse();
    //println!("{}", fs.borrow());
    DepthFirstIteratorFS::from(fs)
        .into_iter()
        .filter(|node| node.borrow().is_dir())
        .map(|n| n.borrow().size())
        .filter(|size| *size <= 100_000)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve_part1(&mut input.lines()), 95437);
    }
}
