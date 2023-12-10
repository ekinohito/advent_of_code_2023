fn extrapolate(slice: &mut [i64]) -> i64 {
    let mut part = slice.len();
    while slice[..part].iter().any(|elem| *elem != 0) {
        for i in 0..part - 1 {
            slice[i] = slice[i + 1] - slice[i];
        }
        part -= 1;
    }

    slice[part..].iter().sum()
}

pub fn solution(input: &str) -> i64 {
    input.lines().map(|line| {
        let mut v: Vec<i64> = line.split_whitespace().map(str::parse).map(Result::unwrap).collect();
        extrapolate(&mut v[..])
    }).sum()
}
