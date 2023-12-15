pub fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |acc, e| {
        acc.wrapping_mul(17).wrapping_add(e.wrapping_mul(17))
    })
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52)
}

pub fn solution(input: &str) -> u64 {
    input.split(',').map(hash).map(|x| x as u64).sum()
}
