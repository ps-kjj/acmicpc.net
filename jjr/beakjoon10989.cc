#include <iostream>
using namespace std;

int main()
{
      ios::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);
	int n;
	cin >> n;
	int cnt[10001] = {0};
	

	for (int i = 0; i < n; i++)
	{
        int a;
		cin >> a;
		cnt[a]++;
	}
	
	for (int i = 1; i < 10001; i++)
	{
		for (int j = 0; j < cnt[i]; j++)
		{
			cout << i << '\n';
		}

	}
	
}
