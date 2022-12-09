use std::collections::HashMap;
use substring::Substring;

pub fn solve(input: &str) -> (u16, u16) {

    let fs = FS::from(input);

    println!("{:?}", fs);

    (0, 0)
}

#[derive(Debug)]
struct FS {
    children: HashMap<String, FS>,
    size: usize,
}
impl FS {
    fn from(input: &str) -> FS {
        let mut root = FS { children: HashMap::new(), size: 0 };
        let mut parent: Option<&mut FS> = None;
        //let mut parent = Box::new(&mut root);
        let mut fs = &mut root;
        for cmd in input.split("$ ") {
            let ccmd = cmd.trim();
            println!("{}", ccmd);
            fs = match ccmd.substring(0, 2) {
                "cd" => {
                    match ccmd.substring(0, 4) {
                        // "cd /" => { &mut root },
                        // "cd ." => { parent.as_mut().unwrap() }, //&mut *fs.parent.as_mut().unwrap() },
                        _ => {
                            println!("[cd] {}", ccmd.substring(3, ccmd.len()));
                            let m = Some(fs);
                            let mut s = FS::file(0);
                            //parent = Some(&mut s);
                            //parent = m;
                            //parent = Some(fs);
                            parent.as_mut().unwrap().children.get_mut(ccmd.substring(3, ccmd.len())).unwrap()
                            //fs
                        },
                    }
                },
                _ => {
                    fs.ls(ccmd.substring(3, ccmd.len()))
                },
            };
        }
        root
    }

    fn ls(&mut self, input: &str) -> &mut Self {
        for line in input.lines() {
            //println!("{}", input);
            let (info, name) = line.split_once(" ").unwrap();
            if info == "dir" {
                self.children.insert(name.to_string(), Self::file(0));
            } else {
                self.children.insert(name.to_string(), Self::file(info.parse::<usize>().unwrap()));
            }
        }
        self
    }

    fn file(size: usize) -> FS {
        FS { children: HashMap::new(), size: size }
    }
}