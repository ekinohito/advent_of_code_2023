

use crate::part_1::hash;

pub fn solution(input: &str) -> u64 {
    let mut hashmap = vec![Vec::<(&str, u64)>::new(); 256];
    for entry in input.split(',') {
        if let Some((key, value)) = entry.split_once('=') {
            let key_hash = hash(key);
            let value: u64 = value.parse().unwrap();
            if let Some((_, val)) = hashmap[key_hash as usize].iter_mut().find(|(x, _)| *x == key) {
                *val = value;
            } else {
                hashmap[key_hash as usize].push((key, value))
            };
        } else {
            let key = entry.trim_end_matches('-');
            let key_hash = hash(key);
            hashmap[key_hash as usize].retain(|(x, _)| *x != key);
        }
    }
    
    let mut result = 0;
    for (i, basket) in hashmap.iter().enumerate() {
        for (j, (_, value)) in basket.iter().enumerate() {
            // println!("Box {i}: #{j} = [{key} {value}]");
            result += (i + 1) * (j + 1) * *value as usize
        }
    }

    result as u64
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#), 145)
}
