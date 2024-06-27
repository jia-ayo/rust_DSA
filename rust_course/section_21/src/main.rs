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

fn bubble_sort(array: &mut Vec<i8>) -> Vec<i8>{
    let mut sorted = true;
    for _ in 1..array.len(){
        sorted = true;
        for j in 0..array.len()-1{
            if array[j] > array[j + 1]{
                array.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted{
            break;
        }
    }
    array.to_vec()
}

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]); //left half
        merge_sort(&mut arr[mid..]); // right side
        merge(arr, mid);
    }

    arr.to_vec()
}

fn quick_sort(arr: &mut [i32], start: usize, end: usize)-> Vec<i32>{
    if start < end {
        let part = partition(arr, start, end);
        quick_sort(arr, start, part-1);
        quick_sort(arr, part + 1, end);
    }
    arr.to_vec()
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let mut i = start;
    let pivot = end;

    for j in start..=end -1 {
        if  arr[j] < arr[pivot]{
            arr.swap(i, j);
            i += 1;
        }
    }
 
    arr.swap(i, pivot);
    i
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec(); //left half to middle
    let right = arr[mid..].to_vec(); //middle too right half

    let mut l = 0;
    let  mut r =0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r] ){
            *val = left[l];
            l += 1;
        } else{
            *val = right[r];
            r += 1;
        }
    }

}

fn main() {
    let mut arr: Vec<i8> = vec![4, 3, 1, 2];
    println!("Before sorting: {:?}", arr);
    selection_sort(&mut arr);
    println!("After sorting {:?}", arr);

    //buble sort
    let mut vec2: Vec<i8> = vec![5, 1, 4, 2, 8];
        bubble_sort(& mut vec2);
    println!("{:?}", vec2);

    //marge sort
    let mut vec = vec![4,7,3,5,1,2];
    merge_sort(&mut vec);
    println!("{:?}", vec);

    //quick sort
    let mut array = vec![8,5,1,2,7,3,4];
    let len = array.len();
    quick_sort(&mut  array, 0, len-1);
    println!("{:?}", array)
}
