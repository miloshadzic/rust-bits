pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    match arr.len() {
        0 => return,
        1 => return,
        n => {
            let mid = n / 2;
            {
                let (left, right) = arr.split_at_mut(mid);

                merge_sort(left);
                merge_sort(right);
            }
            merge(arr, mid)
        }
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], splitpoint: usize) {
    // TODO: Try to do the in-place version
    let mut buf = Vec::new();
    let (mut i, mut j) = (0, splitpoint);

    while buf.len() < arr.len() {
        if i == splitpoint {
            buf.extend_from_slice(&arr[j..]);
            break;
        }

        if j == arr.len() {
            buf.extend_from_slice(&arr[i..splitpoint]);
            break;
        }

        if arr[i] <= arr[j] {
            buf.push(arr[i]);
            i += 1;
        } else {
            buf.push(arr[j]);
            j += 1;
        }

    }

    for (el, bufel) in arr.iter_mut().zip(buf.iter()) {
        *el = *bufel;
    }
}

pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            let t = arr[j].clone();
            arr[j] = arr[j - 1];
            arr[j - 1] = t;
            j -= 1;
        }
    }
}

pub fn quicksort<T: Ord + Copy>(arr: &mut [T]) {
    let lo = 0;
    let hi = arr.len();
    {
        quicksort_sub(arr, lo, hi);
    }
}

fn quicksort_sub<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mut pivot = lo;
    let mut i = lo + 1;

    for j in (lo + 1)..hi {
        if arr[j] < arr[pivot] {
            swap(arr, i, j);
            i += 1;
        }
    }

    pivot = i - 1;
    swap(arr, lo, pivot);
    {
        quicksort_sub(arr, lo, pivot);
        quicksort_sub(arr, pivot + 1, hi);
    }
}

fn swap<T: Copy>(arr: &mut [T], i: usize, j: usize) {
    let t = arr[j];
    arr[j] = arr[i];
    arr[i] = t;
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use self::rand::Rng;
    use std::cmp::Ordering;

    // For testing stability
    #[derive(Eq, Debug, Copy, Clone)]
    struct Pair(i32, i32);
    impl Ord for Pair {
        fn cmp(&self, other: &Pair) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl PartialOrd for Pair {
        fn partial_cmp(&self, other: &Pair) -> Option<Ordering> {
            Some(self.0.cmp(&other.0))
        }
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Pair) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    #[test]
    fn test_mergesort() {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = (0..50).map(|_| rng.gen::<i32>() % 100).collect();
        let mut sorted = vec.clone();
        sorted.sort();
        merge_sort(vec.as_mut_slice());
        assert_eq!(sorted, vec);
    }

    #[test]
    fn test_insertion_sort() {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = (0..50).map(|_| rng.gen::<i32>() % 100).collect();
        let mut sorted = vec.clone();
        sorted.sort();
        insertion_sort(vec.as_mut_slice());
        assert_eq!(sorted, vec);
    }

    #[test]
    fn test_quicksort() {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = (0..50).map(|_| rng.gen::<i32>() % 100).collect();
        let mut sorted = vec.clone();
        sorted.sort();
        quicksort(vec.as_mut_slice());
        assert_eq!(sorted, vec);
    }

    #[test]
    fn test_merge_sort_stability() {
        let mut vec: Vec<Pair> = vec![Pair(6, 3), Pair(5, 5), Pair(6, 1), Pair(1, 3)];
        let mut c = vec.clone();
        c.sort();
        merge_sort(vec.as_mut_slice());
        assert_eq!(c, vec);
    }

    #[test]
    fn test_insertion_sort_stability() {
        let mut vec: Vec<Pair> = vec![Pair(6, 3), Pair(5, 5), Pair(6, 1), Pair(1, 3)];
        let mut c = vec.clone();
        c.sort();
        insertion_sort(vec.as_mut_slice());
        assert_eq!(c, vec);
    }
}
