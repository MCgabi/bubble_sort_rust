fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len()-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        } 
    }

    for nums in arr.iter() {
        println!("{}", nums);
    }
}

fn main() {
    let mut arr = [1135, 12, 342, 134, 234, 3, 4, 5, 12];
    bubble_sort(&mut arr);
}


