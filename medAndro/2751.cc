#include <stdio.h>
#include <malloc.h>
#include <stdlib.h>

int compare(const void* a, const void* b){
	const int* x = (int*)a;
	const int* y = (int*)b;

	if (*x > *y)
		return 1;
	else if (*x < *y)
		return -1;
	return 0;
}

int main() {
	int* arr;
	int size;

	scanf("%d", &size);
	arr = (int*)malloc(sizeof(int) * size);

	for (int i = 0; i < size; i++) {
		scanf("%d", &arr[i]);
	}
	qsort(arr, size, sizeof(arr[0]), compare);
	for (int i = 0; i < size; i++) {
		printf("%d\n", arr[i]);
	}
	return 0;
}