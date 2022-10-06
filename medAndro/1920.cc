#include <stdio.h>
#include <set>
using namespace std;

int main() {
	set<int> nSet;
	int n, m;

	scanf("%d", &n);
	for (int i = 0; i < n; i++) {
		int temp;
		scanf("%d", &temp);
		nSet.insert(temp);
	}
	scanf("%d", &m);
	for (int i = 0; i < m; i++) {
		int temp;
		scanf("%d", &temp);
		if (nSet.count(temp))
			printf("1\n");
		else
			printf("0\n");
	}
	return 0;
}