import java.util.Random;

/**
 *  @brief Main QuickSort class
 */
public class QuickSort {

    /**
     * @brief Main Method, contains logic for testing out quicksort
     * 
     * @param args cli args
     */
    public static void main(String[] args) {
        // Use array size if provided by user, defaults to 1000
        int numArraySize = (args.length > 0) ? Integer.parseInt(args[0]) : 1000;

        // Gen an array w/ x number of rand ints
        Random rand = new Random();
        int[] numArr = new int[numArraySize];

        for (int i = 0; i < numArr.length; i++) {
            // Gen rand int w/ range 0..2000
            numArr[i] = rand.nextInt(200000);
        }

        // Results
        System.out.println("Sorting...");
        // printArray(numArr);

        quickSort(numArr);

        System.out.println("Sorted!");
        // printArray(numArr);

        // Confirm if sorted
        System.out.print("Checking sorted...");
        System.out.println(isSorted(numArr));
    }

    /**
     * @brief Logic for main quickSort method -- base case, choosing a pivot & all that
     * 
     * @param arr     array to be sorted
     * @param lowIdx  low index of said array
     * @param highIdx high index of said array
     */
    public static void quickSort(int[] arr, int lowIdx, int highIdx) {
        // If there's only 1 element in sub-arr, return
        if (lowIdx >= highIdx)
            return;

        // random pivot for better perf on avg | range lowIdx..highIdx
        int pivotIdx = new Random().nextInt(highIdx - lowIdx) + lowIdx;
        int pivot = arr[pivotIdx];

        // swap the val at arr[pivot index] with val arr[high index]
        swap(arr, pivotIdx, highIdx);

        // Do partitioning & get the returned leftPtr
        int leftPtr = partition(arr, lowIdx, highIdx, pivot);

        // sort left & right side of pivot, respectively
        quickSort(arr, lowIdx, leftPtr - 1);
        quickSort(arr, leftPtr + 1, highIdx);
    }

    /**
     * @brief A more natural way to call the method initially
     * 
     * @param arr the array to be sorted
     * @see #quickSort(int[], int, int)
     */
    private static void quickSort(int[] arr) {
        quickSort(arr, 0, arr.length - 1);
    }

    /**
     * @brief This method contains logic for partitioning the sub-arrays
     * 
     * @param arr     array to be partitioned
     * @param lowIdx  low index of said array
     * @param highIdx high index of said array
     * @param pivot   the pivot
     * @return the left pointer
     */
    private static int partition(int[] arr, int lowIdx, int highIdx, int pivot) {
        int leftPtr = lowIdx;
        int rightPtr = highIdx;

        // break out when both pointers meet
        while (leftPtr < rightPtr) {
            // move left_ptr to the right if arr[left pointer]
            // is not > pivot & both pointers haven't met
            while (arr[leftPtr] <= pivot && leftPtr < rightPtr) {
                leftPtr++;
            }

            // move right_ptr to the left if arr[right pointer]
            // is not < pivot & both pointers haven't met
            while (arr[rightPtr] >= pivot && leftPtr < rightPtr) {
                rightPtr--;
            }

            // swap val at arr[left pointer] w/ val at arr[right pointer]
            swap(arr, leftPtr, rightPtr);
        }

        // swap val at arr[left pointer] with arr[high index] (which is pivot) to
        // complete partitioning
        swap(arr, leftPtr, highIdx);
        return leftPtr;
    }

    /**
     * @brief Swaps the value at arr[idx1] w/ arr[idx2] & vice versa.
     * 
     * @param arr  the array to be operated upon
     * @param idx1 index of first element
     * @param idx2 index of second element
     */
    private static void swap(int[] arr, int idx1, int idx2) {
        int temp = arr[idx1];
        arr[idx1] = arr[idx2];
        arr[idx2] = temp;
    }

    /**
     * @brief Just print an integer array...nothing fancy here...
     * 
     * @param arr array to be printed
     */
    private static void printArray(int[] arr) {
        System.out.println("[");
        for (int ele : arr) {
            System.out.print(ele + ", ");

        }
        System.out.println("\n]");
    }

    /**
     * @brief Simple method to check if an int array is sorted
     * 
     * @param arr array to be checked
     * @return true/false indicating wheter sorted or not
     */
    private static boolean isSorted(int[] arr) {
        // just return true if arr is null or if length is < 2
        if (arr == null || arr.length < 2)
            return true;

        for (int i = 0; i < arr.length - 1; i++) {
            // If next element is less than curr element,
            // then it is not sorted.
            if (arr[i] > arr[i + 1])
                return false;
        }
        return true;
    }

}
