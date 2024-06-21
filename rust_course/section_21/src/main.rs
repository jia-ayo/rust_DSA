fn selection_sort(array: &mut Vec<i8>) -> Vec<i8> {
    for i in 0..array.len() - 1 {
        let mut smallest = i;
        for j in (i + 1)..array.len() {
            if array[j] < array[smallest] {
                smallest = j
            }
        }
        array.swap(smallest, i);
    }
    array.to_vec()
}
fn main() {
    let mut arr: Vec<i8> = vec![4, 3, 1, 2];
    println!("Before sorting: {:?}", arr);
    selection_sort(&mut arr);
    println!("After sorting {:?}", arr);

    let mut arr2: Vec<i8> = vec![5, 10, 1, 4,11];
    println!("Before sorting: {:?}", arr2);
    selection_sort(&mut arr2);
    println!("After sorting {:?}", arr2)
}
