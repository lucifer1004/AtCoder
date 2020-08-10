#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n;
  cin >> n;
  vector<int> f(n + 1);
  for (int i = 1; i <= 100; ++i)
    for (int j = 1; j <= 100; ++j)
      for (int k = 1; k <= 100; ++k) {
        int val = i * i + j * j + k * k + i * j + i * k + k * j;
        if (val <= n)
          f[val]++;
      }
  for (int i = 1; i <= n; ++i)
    cout << f[i] << endl;
}