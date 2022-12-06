use std::collections::HashSet;

// Find start of packet with n unique characters.
fn start_of_packet(s: &str, n: usize) -> usize {
    let bytes = s.as_bytes();
    for i in n..s.len() {
        if HashSet::<&u8>::from_iter(bytes[i - n..i].iter()).len() == n {
            return i;
        }
    }
    0
}

// Find start of packet according to part 1 of the problem.
pub fn start_of_packet_p1(s: &str) -> usize {
    start_of_packet(s, 4)
}

// Find start of packet according to part 2 of the problem.
pub fn start_of_packet_p2(s: &str) -> usize {
    start_of_packet(s, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet_4() {
        assert_eq!(start_of_packet_p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet_p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet_p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet_p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_start_of_packet_14() {
        assert_eq!(start_of_packet_p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(start_of_packet_p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_packet_p2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(start_of_packet_p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(start_of_packet_p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
