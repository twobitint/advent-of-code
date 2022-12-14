use std::collections::HashMap;

use substring::Substring;

pub fn solve(input: &str) -> (usize, usize) {
    let fs = build_fs(input);
    let p1 = sum_sizes(&fs, 100000);
    let p2 = space_to_free(&fs, 70000000, 30000000);
    (p1, p2)
}

fn sum_sizes(fs: &FS, max: usize) -> usize {
    fs.arena.iter().fold(0, |a, f| {
        let s = f.size(fs);
        if f.size == 0 && s <= max { a+s } else { a }
    })
}

fn space_to_free(fs: &FS, disk: usize, required: usize) -> usize {
    let diff = required - (disk - fs.get(0).size(fs));
    let mut sizes = fs.arena.iter()
        .filter(|f| f.size == 0)
        .map(|f| { f.size(fs) }).collect::<Vec<_>>();
    sizes.sort();
    for size in sizes {
        if size >= diff { return size; }
    }
    return 0;
}

fn build_fs(input: &str) -> FS {
    let mut fs = FS::new();
    let mut current = fs.file(0);

    for cmd in input.split("$ ") {
        let ccmd = cmd.trim();
        current = match ccmd.substring(0, 2) {
            "cd" => {
                match ccmd.substring(0, 4) {
                    "cd /" => { 0 },
                    "cd ." => { fs.get(current).parent }
                    _ => {
                        let name = ccmd.substring(3, ccmd.len());
                        *(*fs.get(current)).children.get(name).unwrap()
                    },
                }
            },
            _ => {
                let files = ccmd.substring(3, ccmd.len());
                let file = fs.get(current).id;
                ls(&mut fs, file, files);
                current
            },
        };
    }
    fs
}

fn ls(fs: &mut FS, parent: usize, input: &str) {
    for line in input.lines() {
        let (info, name) = line.split_once(" ").unwrap();
        let child;
        if info == "dir" {
            child = fs.file(0);
        } else {
            child = fs.file(info.parse::<usize>().unwrap());
        }
        fs.get_mut(parent).children.insert(name.to_string(), child);
        fs.get_mut(child).parent = parent;
    }
}

struct File {
    id: usize,
    size: usize,
    parent: usize,
    children: HashMap<String, usize>,
}
impl File { 
    fn new(id: usize, size: usize) -> Self { 
        Self { id, size, parent: 0, children: HashMap::new() } 
    }
    fn size(&self, fs: &FS) -> usize {
        self.size + self.children.iter().fold(0, |a, c| { a + fs.get(*c.1).size(fs) })
    }
}

struct FS {
    arena: Vec<File>,
}
impl FS {
    fn new() -> Self { Self { arena: vec![] } }
    fn get(&self, id: usize) -> &File { self.arena.get(id).unwrap() }
    fn get_mut(&mut self, id: usize) -> &mut File { self.arena.get_mut(id).unwrap() }
    fn file(&mut self, size: usize) -> usize {
        let id = self.arena.len();
        self.arena.push(File::new(id, size));
        id
    }
}