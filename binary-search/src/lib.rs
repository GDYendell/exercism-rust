use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<A: AsRef<[V]>, V: Ord>(array: A, key: V) -> Option<usize> {
    let mut window = array.as_ref();
    if window.is_empty() {
        return None;
    }

    let mut window_start = 0;
    loop {
        let pivot = window.len() / 2;

        match key.cmp(&window[pivot]) {
            Equal => return Some(window_start + pivot),
            Less => window = &window[..pivot],
            Greater => {
                window = &window[pivot + 1..];
                window_start += pivot + 1;
            }
        }

        if window.is_empty() {
            return None;
        }
    }
}
