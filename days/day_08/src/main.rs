use std::collections::BTreeSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    corr(n).iter().for_each(|elem| println!("{}", elem));
}

fn corr(n: usize) -> Vec<String> {
    let mut mem: Vec<Vec<String>> = vec![vec!["".into()], vec!["()".into()]];
    for i in 2..=n {
        let mut result = BTreeSet::new();

        for x in &mem[i - 1] {
            result.insert(format!("({})", x));
        }

        for j in 1..i {
            let res1 = &mem[j];
            let res2 = &mem[i - j];
            for x in res1 {
                for y in res2 {
                    result.insert(format!("{}{}", x, y));
                }
            }
        }

        
        mem.push(result.into_iter().collect());
    }
    mem[n].clone()
}
