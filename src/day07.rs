use std::collections::HashMap;
use std::path::PathBuf;

fn parse_output(s: &str) -> Vec<usize> {
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut path = PathBuf::new();

    let lines: Vec<&str> = s.trim().split('\n').collect();
    for line in lines {
        if line.starts_with("$ cd ") {
            let Some((_, dir_name)) = line.rsplit_once(' ') else {
                unreachable!();
            };
            if dir_name == ".." {
                path.pop();
            } else {
                path.push(dir_name)
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            // Do nothing.
        } else {
            let Some((size_str, _)) = line.split_once(' ') else {
                unreachable!();
            };
            let size: usize = size_str.parse().unwrap();
            let mut p = path.clone();
            loop {
                *(sizes.entry(p.clone()).or_insert(0)) += size;
                if !p.pop() {
                    break;
                }
            }
        }
    }

    let mut v = Vec::from_iter(sizes.values().cloned());
    v.sort();
    v
}

pub fn total_size_p1(s: &str) -> usize {
    let v = parse_output(s);
    v.iter().filter(|&&s| s <= 100000).sum()
}

pub fn total_size_p2(s: &str) -> usize {
    let v = parse_output(s);
    let total = 70000000;
    let required = 30000000;
    let used = v.last().unwrap();
    let unused = total - used;
    *v.iter()
        .filter(|&&s| s + unused >= required)
        .take(1)
        .next()
        .unwrap()
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

    #[test]
    fn test_total_size_p2() {
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
        assert_eq!(total_size_p2(input), 24933642)
    }
}
