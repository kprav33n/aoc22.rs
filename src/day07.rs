use std::collections::HashMap;
use std::path::PathBuf;

// Parse output and produce a list of directory sizes.
fn parse_output(s: &str) -> Vec<usize> {
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut path = PathBuf::new();

    let lines: Vec<&str> = s.trim().split('\n').collect();
    for line in lines {
        if line.starts_with("$ cd ") {
            let Some((_, dir_name)) = line.rsplit_once(" ") else {
                unreachable!();
            };
            if dir_name == ".." {
                path.pop();
            } else {
                path.push(dir_name)
            }
        } else if line.starts_with("$ ls") {
            // Do nothing.
        } else if line.starts_with("dir ") {
            // Do nothing.
        } else {
            let Some((size_str, _)) = line.split_once(' ') else {
                unreachable!();
            };
            let size: usize = size_str.parse().unwrap();
            let mut p = path.clone();
            // Logic courtesy: Scott Hutton
            loop {
                *(sizes.entry(p.clone()).or_insert(0)) += size;
                if !p.pop() {
                    break;
                }
            }
        }
    }

    Vec::from_iter(sizes.values().cloned())
}

// Find total size of directories according to part 1 problem statement.
pub fn total_size_p1(s: &str) -> usize {
    let v = parse_output(s);
    v.iter().filter(|&&s| s <= 100000).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_size_p1() {
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
        assert_eq!(total_size_p1(input), 95437)
    }
}
