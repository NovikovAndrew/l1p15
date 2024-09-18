fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    // Сортировка левой части (до опорного элемента)
    quicksort(&mut left[..pivot_index]);
    // Сортировка правой части (после опорного элемента)
    quicksort(&mut right[..]);
}

// Разделение массива на две части относительно опорного элемента
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    let pivot_value = arr[pivot_index].clone();  // Клонируем опорное значение
    arr.swap(pivot_index, len - 1); // Перемещаем опорный элемент в конец
    let pivot = &arr[len - 1]; // Теперь опорный элемент находится в конце

    let mut i = 0;
    for j in 0..(len - 1) {
        if arr[j] < pivot_value { // Используем клонированное значение для сравнения
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1); // Перемещаем опорный элемент на свое место
    i
}

fn main() {
    let mut vec = vec![3, 6, 8, 10, 1, 2, 1];
    println!("До сортировки: {:?}", vec);

    quicksort(&mut vec);

    println!("После сортировки: {:?}", vec);
}
