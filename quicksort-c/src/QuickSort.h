#ifndef QUICKSORT_H
#define QUICKSORT_H

#define bool int
#define false 0
#define true 1

void quickSort(int arr[], int lowIdx, int highIdx);
int partition(int arr[], int lowIdx, int highIdx, int pivot);
void swap(int *ptr1, int *ptr2);
void printArray(int arr[], size_t arrSize);
bool isSorted(int arr[], size_t arrSize);

#endif