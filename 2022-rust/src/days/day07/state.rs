use std::error::Error;
use std::rc::Rc;

use super::fs::{Directory, Entry};

pub struct Session {
    root: Rc<Directory>,
    cwd: Rc<Directory>,
}

impl Session {
    pub fn move_to_root(&mut self) {
        self.cwd = self.root.clone();
    }
    pub fn move_to_parent(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.cwd.parent.upgrade() {
            self.cwd = parent;

            Ok(())
        } else {
            Err(Box::from("parent has been freed"))
        }
    }
    pub fn move_to_child(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.cwd = self.assert_directory(name)?;

        Ok(())
    }
    pub fn assert_directory(&mut self, name: &str) -> Result<Rc<Directory>, Box<dyn Error>> {
        self.cwd.assert_directory(name)
    }
    pub fn assert_file(&mut self, name: &str, size: u32) -> Result<(), Box<dyn Error>> {
        self.cwd.assert_file(name, size)
    }

    pub fn small_dirs_total(&self, threshold: u32) -> u32 {
        small_dirs_total(&self.root, threshold)
    }
    pub fn to_delete_size(&self, max_used: u32) -> u32 {
        let size = self.root.size();
        if size <= max_used {
            return 0;
        }

        smallest_larger_than(&self.root, size - max_used)
    }

    pub fn dump_fs(&self) {
        self.root.dump(0, "/");
    }
}

impl Default for Session {
    fn default() -> Self {
        let root = Directory::new_root();
        Self {
            root: root.clone(),
            cwd: root,
        }
    }
}

fn small_dirs_total(dir: &Rc<Directory>, threshold: u32) -> u32 {
    let size = dir.size();
    let mut total: u32 = if size < threshold { size } else { 0 };
    for entry in dir.children.borrow().values() {
        if let Entry::Directory(subdir) = entry {
            total += small_dirs_total(subdir, threshold);
        }
    }

    total
}

fn smallest_larger_than(dir: &Rc<Directory>, threshold: u32) -> u32 {
    let mut size = dir.size();
    if size < threshold {
        return u32::MAX;
    }
    for entry in dir.children.borrow().values() {
        if let Entry::Directory(subdir) = entry {
            let sub_size = smallest_larger_than(subdir, threshold);
            if sub_size < size {
                size = sub_size;
            }
        }
    }

    size
}
