use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::{Rc, Weak};

pub struct Directory {
    pub parent: Weak<Directory>,
    current: Weak<Directory>,
    pub children: RefCell<HashMap<String, Entry>>,
}

impl Directory {
    fn new(parent: Weak<Self>, current: Weak<Self>) -> Self {
        Self {
            parent,
            current,
            children: RefCell::new(HashMap::new()),
        }
    }
    pub fn new_root() -> Rc<Self> {
        Rc::new_cyclic(|it| Self::new((*it).to_owned(), (*it).to_owned()))
    }
    fn new_child(parent: &Weak<Self>) -> Rc<Self> {
        Rc::new_cyclic(|it| Self::new(parent.to_owned(), (*it).to_owned()))
    }
    pub fn size(&self) -> u32 {
        let mut size: u32 = 0;
        for entry in self.children.borrow().values() {
            size += entry.size();
        }

        size
    }
    pub fn assert_directory(&self, name: &str) -> Result<Rc<Directory>, Box<dyn Error>> {
        if let Some(entry) = self.children.borrow().get(name) {
            return if let Entry::Directory(directory) = entry {
                Ok(directory.to_owned())
            } else {
                Err(Box::from(
                    "entry already exists with type other than directory",
                ))
            };
        }

        let new_directory = Directory::new_child(&self.current);
        let new_entry = Entry::Directory(new_directory.clone());
        self.children
            .borrow_mut()
            .insert(name.to_owned(), new_entry);

        Ok(new_directory)
    }
    pub fn assert_file(&self, name: &str, size: u32) -> Result<(), Box<dyn Error>> {
        if let Some(entry) = self.children.borrow().get(name) {
            return if let Entry::File { size: ent_size } = entry {
                if *ent_size == size {
                    Ok(())
                } else {
                    Err(Box::from("file entry already exists with other size"))
                }
            } else {
                Err(Box::from("entry already exists with type other than file"))
            };
        }

        let new_entry = Entry::File { size };
        self.children
            .borrow_mut()
            .insert(name.to_owned(), new_entry);

        Ok(())
    }
    pub fn dump(&self, level: usize, name: &str) {
        println!("{:indent$}- {} (dir)", "", name, indent = level);
        for (c_name, entry) in self.children.borrow().iter() {
            match entry {
                Entry::File { size } => println!(
                    "{:indent$}  - {} (file, size={})",
                    "",
                    c_name,
                    size,
                    indent = level
                ),
                Entry::Directory(dir) => dir.dump(level + 2, c_name.as_str()),
            }
        }
    }
}

pub enum Entry {
    File { size: u32 },
    Directory(Rc<Directory>),
}

impl Entry {
    pub fn size(&self) -> u32 {
        match self {
            Self::File { size } => *size,
            Self::Directory(dir) => dir.size(),
        }
    }
}
