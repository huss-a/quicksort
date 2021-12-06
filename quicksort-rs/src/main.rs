use rand::Rng;

mod quicksort {
    //! Main quicksort module
    use super::Rng;

    /// Logic for main quickSort method -- base case, choosing a pivot & all that
    ///
    /// # Arguments:
    /// * `arr`: array to be sorted
    /// * `low_idx`  low index of said array
    /// * `high_idx` high index of said array
    ///
    /// # Examples
    /// ```
    /// let lucky_nums = vec![-23, 1, 3, 33, 1, 20, 2, -10];
    /// quicksort(lucky_nums, 0, lucky_nums.len() - 1);
    /// println!("{:?}", lucky_nums); // sorted
    /// ```
    pub fn quicksort(arr: &mut Vec<isize>, low_idx: isize, high_idx: isize) {
        // If there's only 1 element in sub-arr, return
        if low_idx >= high_idx {
            return;
        }

        // random pivot for better perf on avg | range low_idx..high_idx
        let mut rand = rand::thread_rng();
        let pivot_idx = rand.gen_range(low_idx..high_idx) as usize;
        let pivot = arr[pivot_idx];

        // swap the val at arr[pivot index] with val arr[high index]
        arr.swap(pivot_idx, high_idx as usize);

        // Do partitioning & get the returned left_ptr
        let left_ptr = partition(arr, low_idx, high_idx, pivot);

        // sort left & right side of pivot, respectively
        quicksort(arr, low_idx, (left_ptr - 1) as isize);
        quicksort(arr, left_ptr + 1, high_idx);
    }

    /// This method contains logic for partitioning the sub-arrays
    ///
    /// # Arguments:
    ///
    /// * `arr`:     array to be partitioned
    /// * `low_idx`:  low index of said array
    /// * `high_idx`: high index of said array
    /// * `pivot`:   the pivot
    /// # Returns:
    ///  the left pointer
    fn partition(arr: &mut Vec<isize>, low_idx: isize, high_idx: isize, pivot: isize) -> isize {
        let mut left_ptr = low_idx as isize;
        let mut right_ptr = high_idx as isize;

        // break out when both pointers meet
        while left_ptr < right_ptr {
            // move left_ptr to the right if arr[left pointer]
            // is not > pivot & both pointers haven't met
            while arr[left_ptr as usize] <= pivot && left_ptr < right_ptr {
                left_ptr += 1;
            }

            // move right_ptr to the left if arr[right pointer]
            // is not < pivot & both pointers haven't met
            while arr[right_ptr as usize] >= pivot && left_ptr < right_ptr {
                right_ptr -= 1;
            }

            // swap val at arr[left pointer] w/ val at arr[right pointer]
            arr.swap(left_ptr as usize, right_ptr as usize);
        }

        // complete partitioning
        arr.swap(left_ptr as usize, high_idx as usize);
        left_ptr
    }

    /// Simple method to check if a Vec\<isize\> is sorted
    ///
    /// # Arguments
    /// * `arr`: array to be checked
    ///
    /// # Returns
    /// true/false indicating wheter sorted or not
    ///
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

    // Use array size if provided by user, defaults to 1000
    let num_arr_size = {
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            1000
        } else {
            args[1].parse::<u32>().unwrap() as usize
        }
    };

    // Gen an array w/ x number of rand ints
    let mut num_arr = Vec::with_capacity(num_arr_size);
    let mut rand = rand::thread_rng();
    for _ in 0..num_arr_size {
        // Gen rand int
        num_arr.push(rand.gen_range(-100_000..200_000));
    }

    // Results
    println!("Sorting...");
    // println!("{:?}", arr);

    quicksort::quicksort(&mut num_arr, 0, (num_arr_size - 1) as isize);

    println!("Sorted");
    // println!("{:?}", arr);

    // Confirm if sorted
    println!("Checking sorted..{}", quicksort::is_sorted(&num_arr));
}
