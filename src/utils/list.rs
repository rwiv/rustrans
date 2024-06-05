pub fn split_vec_move<T>(vec: &mut Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut sub = Vec::new();
    for _ in 0..vec.len() {
        sub.push(vec.remove(0));
        if sub.len() == size {
            result.push(sub);
            sub = Vec::new();
        }
    }
    result.push(sub);
    result
}

pub fn split_vec_ref<T>(vec: &Vec<T>, size: usize) -> Vec<Vec<&T>> {
    let mut result = Vec::new();
    let mut sub = Vec::new();
    for i in 0..vec.len() {
        sub.push(&vec[i]);
        if sub.len() == size {
            result.push(sub);
            sub = Vec::new();
        }
    }
    result.push(sub);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_vec() {
        let mut v = vec![
            str("1"), str("2"), str("3"), str("4"), str("5"),
            str("6"), str("7"), str("8"), str("9"), str("10"),
        ];
        let size = 3;

        let splits = split_vec_ref(&mut v, size);

        for (i, split) in splits.iter().enumerate() {
            println!("Sub-vector {}: {:?}", i, split);
        }
        println!("{:?}", v);
    }

    fn str(s: &str) -> String { String::from(s) }
}