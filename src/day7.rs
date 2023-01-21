use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::cell::Ref;

use regex::Regex;
use trees::{RcNode, Tree};

#[derive(Debug)]
struct File {
    size: u32,
    name: String
}

#[derive(Debug)]
struct Dir {
    name: String
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

impl File {
    pub fn new(size: u32, name: &str) -> Self {
        Self { size, name: name.to_owned() }
    }
}

#[derive(Debug)]
enum Inode {
    Dir(Dir),
    File(File)
}

pub fn a(data: &str) {
    let mut filesystem = Tree::new(Inode::Dir(Dir::new("/")));
    // This from() statement will default to the root of the tree
    let mut current_inode = RcNode::from(filesystem);
    let ls_re = Regex::new(r"\$ ls").unwrap();
    let cd_re = Regex::new(r"\$ cd ([^ ]+)?").unwrap();
    let dir_re = Regex::new(r"dir ([^ ]+)").unwrap();
    let file_re = Regex::new(r"(\d+) ([^ ]+)").unwrap();
    for line in data.lines() {
        if let Some(_) = ls_re.captures(line) {
            // Do nothing, because we don't really care...
        } else if let Some(re) = cd_re.captures(line) {
            let target = re.get(1).unwrap().as_str();
            match target {
                ".." => current_inode = current_inode.parent().unwrap(),
                "/" => current_inode = RcNode::from(filesystem),
                subdir => {
                    for child in current_inode.iter_rc() {
                        let copy = child.clone().data();
                        if let Inode::Dir(copy ) = *child.clone().data() {
                                current_inode = child;
                                break;
                            }
                    }
                }
            }
            println!("PWD is now {:?}", current_inode.data());
        } else if let Some(re) = dir_re.captures(line) {
            println!("Dir");
            current_inode.push_back(Tree::new(Inode::Dir(Dir::new(re.get(1).unwrap().as_str()))));
        } else if let Some(re) = file_re.captures(line) {
            let size: u32 = re.get(1).unwrap().as_str().parse().unwrap();
            let filename = re.get(2).unwrap().as_str();
            println!("File: {filename} {size}");
        }
        else {
            panic!("No regex matches: {line}");
        }
    }

}

pub fn b(data: &str) {

}