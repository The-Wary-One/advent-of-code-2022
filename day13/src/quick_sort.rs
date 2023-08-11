pub fn quick_sort<T: Ord>(s: &mut [T]) {
    let pivot_idx = if s.len() > 1 {
        s.len() - 1
    } else {
        return;
    };

    let mut lo = 0;
    for i in 0..pivot_idx {
        if s[i].le(&s[pivot_idx]) {
            s.swap(i, lo);
            lo += 1;
        }
    }

    s.swap(pivot_idx, lo);

    quick_sort(&mut s[..lo]);
    quick_sort(&mut s[lo + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut input = [8, 7, 5, 4, 6];
        quick_sort(&mut input);
        assert_eq!(input, [4, 5, 6, 7, 8]);
    }
}
