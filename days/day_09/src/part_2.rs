fn extrapolate(slice: &mut [i64]) -> i64 {
    let mut part = 0;
    while slice[part..].iter().any(|elem| *elem != 0) {
        // dbg!(&slice);
        for i in (part + 1..slice.len()).rev() {
            slice[i] = slice[i] - slice[i - 1];
        }
        part += 1;
    }

    slice[..part].iter().rev().fold(0, |acc, elem| elem - acc)
}

#[test]
fn feature() {
    dbg!(solution("10 13 16 21 30 45"));
}

pub fn solution(input: &str) -> i64 {
    input.lines().map(|line| {
        let mut v: Vec<i64> = line.split_whitespace().map(str::parse).map(Result::unwrap).collect();
        extrapolate(&mut v[..])
    }).sum()
}

