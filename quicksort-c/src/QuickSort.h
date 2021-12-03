#ifndef QUICKSORT_H
#define QUICKSORT_H

#define bool int
#define false 0
#define true 1

/**
 * @file 
 * Header file for QuickSort.c
 * @author Hussain
 */

/**
 * Logic for main quickSort method -- base case, choosing a pivot & all that
 *
 * @param arr     array to be sorted
 * @param lowIdx  low index of said array
 * @param highIdx high index of said array
 */
void quickSort(int arr[], int lowIdx, int highIdx);

/**
 * This method contains logic for partitioning the sub-arrays
 *
 * @param arr     array to be partitioned
 * @param lowIdx  low index of said array
 * @param highIdx high index of said array
 * @param pivot   the pivot
 * @return the left pointer
 */
int partition(int arr[], int lowIdx, int highIdx, int pivot);

/**
 * Swaps the value at arr[idx1] w/ arr[idx2] & vice versa.
 *
 * @param ptr1 pointer to first element
 * @param ptr2 pointer to second element
 */
void swap(int *ptr1, int *ptr2);

/**
 * Just print an integer array...nothing fancy here...
 *
 * @param arr array to be printed
 * @param arrSize array's size
 */
void printArray(int arr[], size_t arrSize);

/**
 * Simple method to check if an int array is sorted
 *
 * @param arr array to be checked
 * @param arrSize array's size
 * @return true/false indicating wheter sorted or not
 */
bool isSorted(int arr[], size_t arrSize);

#endif