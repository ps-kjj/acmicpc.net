#include <stdio.h>

int main() {
	int size;
	int arr[10000] = {0,};

	scanf("%d", &size);

	for (int i = 0; i < size; i++) {
		int temp;
		scanf("%d", &temp);
		arr[temp-1] += 1;
	}

	for (short i = 0; i < 10000; i++) {
		if (arr[i]) {
			for (int j = 0; j < arr[i]; j++) {
				printf("%d\n", i+1);
			}
		}
	}
	return 0;
}