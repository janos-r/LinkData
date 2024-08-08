use super::*;

#[test]
#[rustfmt::skip] // highlight overlap
fn overlap_6() {
    let a = [6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3];
    let b =                [7, 1, 2, 7, 3, 3, 1, 1, 1];
    assert_eq!(find_overlap(2, &a, &b), 6);
}

#[test]
#[rustfmt::skip] // highlight overlap
fn merged_6() {
    let a = vec!            [6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3];
    let b = vec!                           [7, 1, 2, 7, 3, 3, 1, 1, 1];
    assert_eq!(merge(a, b), [6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3, 1, 1, 1]);
}

#[test]
fn merged_on_threshold() {
    let a = vec![1, 2, 3, 4];
    let b = vec![3, 4, 5, 6];
    assert_eq!(merge(a, b), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn merged_none() {
    let a = vec![5, 5, 1, 2];
    let b = vec![5, 1, 7, 8];
    assert_eq!(merge(a, b), [5, 5, 1, 2, 5, 1, 7, 8]);
}

#[test]
fn overlap_all() {
    let a = vec![5, 5, 5, 5];
    let b = vec![5, 5, 5, 5];
    assert_eq!(find_overlap(2, &a, &b), 4);
}

#[test]
fn merged_exact() {
    let a = vec![5, 5, 5, 5];
    let b = vec![5, 5, 5, 5];
    assert_eq!(merge(a, b), [5, 5, 5, 5]);
}

#[test]
fn merged_all_a() {
    let a = vec![5, 5, 5];
    let b = vec![5, 5, 5, 5];
    assert_eq!(merge(a, b), [5, 5, 5, 5]);
}

#[test]
fn merged_all_b() {
    let a = vec![5, 5, 5, 5];
    let b = vec![5, 5, 5];
    assert_eq!(merge(a, b), [5, 5, 5, 5]);
}

#[test]
fn repeating_pattern() {
    let a = vec![1, 2, 1, 2];
    let b = vec![1, 2, 1, 2];
    assert_eq!(merge(a, b), [1, 2, 1, 2]);
}

#[test]
fn long_array_iter() {
    let a = Vec::from([0; 200_000]);
    let b = Vec::from([5; 200_000]);
    assert_eq!(merge(a, b).len(), 400_000);
}
