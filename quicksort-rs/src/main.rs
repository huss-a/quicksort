use rand::Rng;

mod quicksort {
    use super::Rng;

    pub fn quicksort(arr: &mut Vec<isize>, low_idx: isize, high_idx: isize) {
        if low_idx >= high_idx {
            return;
        }
        let mut rand = rand::thread_rng();
        let pivot_idx = rand.gen_range(low_idx..high_idx) as usize;
        let pivot = arr[pivot_idx];
        arr.swap(pivot_idx, high_idx as usize);
        let left_ptr = partition(arr, low_idx, high_idx, pivot);
        quicksort(arr, low_idx, (left_ptr - 1) as isize);
        quicksort(arr, left_ptr + 1, high_idx);
    }

    fn partition(arr: &mut Vec<isize>, low_idx: isize, high_idx: isize, pivot: isize) -> isize {
        let mut left_ptr = low_idx as isize;
        let mut right_ptr = high_idx as isize;

        while left_ptr < right_ptr {
            while arr[left_ptr as usize] <= pivot && left_ptr < right_ptr {
                left_ptr += 1;
            }

            while arr[right_ptr as usize] >= pivot && left_ptr < right_ptr {
                right_ptr -= 1;
            }

            arr.swap(left_ptr as usize, right_ptr as usize);
        }
        arr.swap(left_ptr as usize, high_idx as usize);
        left_ptr
    }

    pub fn is_sorted(arr: &[isize]) -> bool {
        if arr.len() < 2 {
            return true;
        }
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    use std::env;

    let num_arr_size = {
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            1000
        } else {
            args[1].parse::<u32>().unwrap() as usize
        }
    };

    let mut num_arr = Vec::with_capacity(num_arr_size);
    let mut rand = rand::thread_rng();
    for _ in 0..num_arr_size {
        num_arr.push(rand.gen_range(-100_000..200_000));
    }
    println!("Sorting...");
    // println!("{:?}", arr);
    quicksort::quicksort(&mut num_arr, 0, (num_arr_size - 1) as isize);
    println!("Sorted");
    // println!("{:?}", arr);
    println!("Checking sorted..{}", quicksort::is_sorted(&num_arr));
}
