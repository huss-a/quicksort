import random
import sys


class QuickSort:
    """
    Main QuickSort class containing methods
    """

    def quicksort(self, arr: [int], low_idx: int, high_idx: int):
        """
        Logic for main quickSort method -- base case, choosing a pivot & all that

        Args:
            arr:     array to be sorted
            low_idx:  low index of said array
            high_idx: high index of said array
        """
        # If there's only 1 element in sub-arr, return
        if low_idx >= high_idx:
            return

        # random pivot for better perf on avg | range low_idx..high_idx
        pivot_idx = random.randint(0, high_idx - low_idx) + low_idx
        pivot = arr[pivot_idx]

        # swap the val at arr[pivot index] with val arr[high index]
        self.swap(arr, pivot_idx, high_idx)

        # Do partitioning & get the returned left_ptr
        left_ptr = self.partition(arr, low_idx, high_idx, pivot)

        # sort left & right side of pivot, respectively
        self.quicksort(arr, low_idx, left_ptr - 1)
        self.quicksort(arr, left_ptr + 1, high_idx)

    def partition(self, arr: [int], low_idx: int, high_idx: int, pivot: int) -> int:
        """
        This method contains logic for partitioning the sub-arrays

        Args:
            arr:     array to be partitioned
            low_idx:  low index of said array
            high_idx: high index of said array
            pivot:   the pivot
        Returns:
            the left pointer
        """
        left_ptr = low_idx
        right_ptr = high_idx

        # break out when both pointers meet
        while left_ptr < right_ptr:
            # move left_ptr to the right if arr[left pointer]
            # is not > pivot & both pointers haven't met
            while arr[left_ptr] <= pivot and left_ptr < right_ptr:
                left_ptr += 1

            # move right_ptr to the left if arr[right pointer]
            # is not < pivot & both pointers haven't met
            while arr[right_ptr] >= pivot and left_ptr < right_ptr:
                right_ptr -= 1

            # swap val at arr[left pointer] w/ val at arr[right pointer]
            self.swap(arr, left_ptr, right_ptr)
        # swap val at arr[left pointer] with arr[high index] (which is pivot) to
        # complete partitioning
        self.swap(arr, left_ptr, high_idx)
        return left_ptr

    def swap(self, arr: [int], idx1: int, idx2: int) -> None:
        """
        Swaps the value at arr[idx1] w/ arr[idx2] & vice versa.

        Args:
            arr:  the array to be operated upon
            idx1: index of first element
            idx2: index of second element
        """
        temp = arr[idx1]
        arr[idx1] = arr[idx2]
        arr[idx2] = temp

    def is_sorted(self, arr: [int]) -> bool:
        """
        Simple method to check if an int array is sorted
        Args:
            arr: array to be checked
        Returns:
            true/false indicating wheter sorted or not
        """
        # just return true if arr is None or if length is < 2
        if len(arr) < 2 or arr == None:
            return True

        for i in range(0, len(arr) - 1, 1):
            # If next element is less than curr element,
            # then it is not sorted.
            if arr[i] > arr[i+1]:
                return False

        return True


if __name__ == "__main__":
    num_arr_size = 1000

    # Use array size if provided by user, defaults to 1000
    if sys.argv[1]:
        num_arr_size = int(sys.argv[1])

    num_arr = []
    qs = QuickSort()

    # Gen an array w/ x number of rand ints
    for i in range(num_arr_size):
        num_arr.append(random.randint(0, 200000))

    # Results
    print("Sorting...")
    # print(num_arr)

    qs.quicksort(num_arr, 0, len(num_arr) - 1)

    print("Sorted!")
    # print(num_arr)

    # Confirm if sorted
    print("Checking sorted...")
    print(qs.is_sorted(num_arr))
