fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

// Разделение массива на две части относительно опорного элемента
fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

fn main() {
    let mut vec = vec![3, 6, 8, 10, 1, 2, 1];
    println!("До сортировки: {:?}", vec);

    quicksort(&mut vec);

    println!("После сортировки: {:?}", vec);
}
