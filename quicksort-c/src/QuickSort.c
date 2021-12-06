#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "QuickSort.h"

int main(int argc, char *argv[])
{
    // init random num generator
    srand(time(NULL));

    // Use array size if provided by user, defaults to 1000
    int numArrSize = (argc >= 2) ? atoi(argv[1]) : 1000;

    // Gen an array w/ x number of rand ints
    int numArr[numArrSize];

    for (int i = 0; i < numArrSize; i++)
    {
        // Gen rand int
        numArr[i] = rand() % 20000;
    }

    // Results
    printf("Sorting...\n");
    // printArray(numArr, numArrSize);

    quickSort(numArr, 0, numArrSize - 1);

    printf("Sorted!\n");
    // printArray(numArr, numArrSize);

    // Confirm if sorted
    printf("Checking sorted...");
    printf("%s\n", (isSorted(numArr, numArrSize) ? "True" : "False"));
}

void quickSort(int arr[], int lowIdx, int highIdx)
{
    // If there's only 1 element in sub-arr, return
    if (lowIdx >= highIdx)
        return;

    // implementing choosing a rand pivot is kinda hard
    // in C unfortunately ;( ...memory issues & segfaults :-/
    int pivot = arr[highIdx];

    // Do partitioning & get the returned leftPtr
    int leftPtr = partition(arr, lowIdx, highIdx, pivot);

    // sort left & right side of pivot, respectively
    quickSort(arr, lowIdx, leftPtr - 1);
    quickSort(arr, leftPtr + 1, highIdx);
}

int partition(int arr[], int lowIdx, int highIdx, int pivot)
{
    int leftPtr = lowIdx;
    int rightPtr = highIdx;

    // break out when both pointers meet
    while (leftPtr < rightPtr)
    {
        // move left_ptr to the right if arr[left pointer]
        // is not > pivot & both pointers haven't met
        while (arr[leftPtr] <= pivot && leftPtr < rightPtr)
        {
            leftPtr++;
        }

        // move right_ptr to the left if arr[right pointer]
        // is not < pivot & both pointers haven't met
        while (arr[rightPtr] >= pivot && leftPtr < rightPtr)
        {
            rightPtr--;
        }

        // swap val at arr[left pointer] w/ val at arr[right pointer]
        swap(&arr[leftPtr], &arr[rightPtr]);
    }

    // swap val at arr[left pointer] with arr[high index] (which is pivot) to
    // complete partitioning
    swap(&arr[leftPtr], &arr[highIdx]);
    return leftPtr;
}

void swap(int *ptr1, int *ptr2)
{
    int temp = *ptr1;
    *ptr1 = *ptr2;
    *ptr2 = temp;
}

void printArray(int arr[], size_t arrSize)
{
    printf("[\n");
    for (int i = 0; i < arrSize; i++)
    {
        printf("%d, ", arr[i]);
    }
    printf("\n]\n");
}

bool isSorted(int arr[], size_t arrSize)
{
    // just return true if arr is NULL or if length is < 2
    if (arr == NULL || arrSize < 2)
        return true;

    for (int i = 0; i < arrSize - 1; i++)
    {
        // If next element is less than curr element,
        // then it is not sorted.
        if (arr[i] > arr[i + 1])
            return false;
    }
    return true;
}