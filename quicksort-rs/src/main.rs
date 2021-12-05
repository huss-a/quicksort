#![allow(non_snake_case)]
#![allow(dead_code)]

use rand::Rng;

mod quicksort {
    use super::Rng;

    pub fn quicksort(arr: &mut [isize], lowIdx: isize, highIdx: isize) {
        if lowIdx >= highIdx {
            return;
        }
        let mut rand = rand::thread_rng();
        let pivotIdx = rand.gen_range(lowIdx..highIdx) as usize;
        let pivot = arr[pivotIdx];
        arr.swap(pivotIdx, highIdx as usize);
        let leftPtr = partition(arr, lowIdx, highIdx, pivot);
        quicksort(arr, lowIdx, (leftPtr - 1) as isize);
        quicksort(arr, leftPtr + 1, highIdx);
    }

    fn partition(arr: &mut [isize], lowIdx: isize, highIdx: isize, pivot: isize) -> isize {
        let mut leftPtr = lowIdx as isize;
        let mut rightPtr = highIdx as isize;

        while leftPtr < rightPtr {
            while arr[leftPtr as usize] <= pivot && leftPtr < rightPtr {
                leftPtr += 1;
            }

            while arr[rightPtr as usize] >= pivot && leftPtr < rightPtr {
                rightPtr -= 1;
            }

            arr.swap(leftPtr as usize, rightPtr as usize);
        }
        arr.swap(leftPtr as usize, highIdx as usize);
        leftPtr
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
    const NUM_ARR_SIZE: usize = 10000000;
    let mut arr: [isize; NUM_ARR_SIZE] = [0; NUM_ARR_SIZE];
    let mut rand = rand::thread_rng();
    for i in 0..NUM_ARR_SIZE {
        arr[i] = rand.gen_range(0..200000);
    }
    println!("Sorting...");
    // println!("{:?}", arr);
    quicksort::quicksort(&mut arr, 0, (NUM_ARR_SIZE - 1) as isize);
    println!("Sorted");
    // println!("{:?}", arr);
    println!("Checking sorted..{}", quicksort::is_sorted(&arr));
}
