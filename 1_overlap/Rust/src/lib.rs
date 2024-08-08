fn find_overlap(min_len: usize, a: &[i32], b: &[i32]) -> usize {
    let a_len = a.len();
    let max_len = std::cmp::min(a_len, b.len());
    // the .rev() causes a non-greedy pattern search, matching as much as possible
    // but slower on long lists with short matches
    for i in (min_len..=max_len).rev() {
        if a[a_len - i..] == b[..i] {
            return i;
        }
    }
    0
}

pub fn merge(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    let overlap = find_overlap(2, &a, &b);
    a.truncate(a.len() - overlap);
    a.append(&mut b);
    a
}

#[cfg(test)]
mod test;
