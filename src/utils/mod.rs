pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v.iter().map(|e| e.len()).max().unwrap_or(0);
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .flat_map(|n| n.next())
                .collect::<Vec<T>>()
        })
        .collect()
}