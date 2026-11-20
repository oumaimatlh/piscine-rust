pub fn bubble_sort(arr: &mut [i32]) {
    for _ in 0..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                let n = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = n;
            }
        }
    }
}