use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;

struct Directory {
    name: PathBuf,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Directory {
    pub fn new(name: PathBuf) -> Directory {
        return Directory { name: name, dirs: Vec::new(), files: Vec::new() }
    }

    pub fn add_dir(&mut self, dir: PathBuf) {
        self.dirs.push(dir);
    }

    pub fn add_file(&mut self, file: PathBuf) {
        self.files.push(file);
    }
}

struct AFile {
    name: PathBuf,
    size: u32,
}

impl AFile {
    pub fn new(name: PathBuf, size: u32) -> AFile {
        return AFile { name, size }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut directories: HashMap<PathBuf, Directory> = HashMap::new();
    directories.insert(PathBuf::from("/"), Directory::new(PathBuf::from("/")));
    let mut files: HashMap<PathBuf, AFile> = HashMap::new();
    let mut current_path: PathBuf = PathBuf::new();
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                if text != "" {            
                    
                    let mut split = text.split(" ").map(String::from);
                    let first_word = split.next().unwrap();
                    
                    match first_word.as_str() {
                        "$" => {
                            if split.next().unwrap() == "cd" {
                                let current_directory = split.next().unwrap();
                                if current_directory == ".." {
                                    current_path = current_path.parent().unwrap().to_path_buf();
                                } else {
                                    current_path = current_path.join(current_directory);
                                }
                            }
                        },
                        "dir" => {
                            let dir_name = split.next().unwrap();
                            let curr_dir = directories.get_mut(&current_path).unwrap();
                            curr_dir.add_dir(current_path.join(&dir_name));
                            directories.insert(current_path.join(&dir_name), Directory::new(PathBuf::from(dir_name)));
                        },
                        num => {
                            let file_name = split.next().unwrap();
                            let curr_dir = directories.get_mut(&current_path).unwrap();
                            curr_dir.add_file(current_path.join(&file_name));
                            files.insert(current_path.join(&file_name), AFile::new(PathBuf::from(&file_name), num.parse::<u32>().unwrap()));
                        },
                    }

                }
            }
        }
    }

    let mut res: Vec<u32> = Vec::new();

    fn add_to_res(dir: &Directory, files: &HashMap<PathBuf, AFile>, directories: &HashMap<PathBuf, Directory>, res: &mut Vec<u32>) -> u32 {
        let mut size = 0;
        for file_name in dir.files.iter() {
            let file = files.get(file_name).unwrap();
            size += file.size;
        }
        for dir_name in dir.dirs.iter() {
            let sub_dir = directories.get(dir_name).unwrap();
            size += add_to_res(sub_dir, files, directories, res);
        }

        res.push(size);

        return size;
    }

    let n = add_to_res(directories.get(&PathBuf::from("/")).unwrap(), &files, &directories, &mut res);
    let diff = n - 40000000;

    let mut final_res = 70000000;
    for x in res {
        if x >= diff && x <= final_res {
            final_res = x;
        }
    }

    println!("The result is {}", final_res);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
