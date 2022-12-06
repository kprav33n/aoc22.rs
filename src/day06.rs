use std::collections::HashSet;

pub fn start_of_packet(s: &str) -> usize {
    let bytes = s.as_bytes();
    for i in 4..s.len() {
        if HashSet::<&u8>::from_iter(bytes[i - 4..i].iter()).len() == 4 {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
