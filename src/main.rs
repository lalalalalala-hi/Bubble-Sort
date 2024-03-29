fn bubble_sort_fixed(arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn bubble_sort_generic<T: Ord>(arr: &mut [T]) {
    let mut n = arr.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn main() {
    let mut arr = [5, 3, 2, 4, 1];
    bubble_sort_fixed(&mut arr);
    println!("{:?}", arr);

    let mut arr = [5, 3, 2, 4, 1];
    bubble_sort_generic(&mut arr);
    println!("{:?}", arr);

    let mut arr = ['e', 'd', 'c', 'b', 'a'];
    bubble_sort_generic(&mut arr);
    println!("{:?}", arr);
}
