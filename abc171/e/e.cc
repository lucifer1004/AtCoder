#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n;
  cin >> n;
  vector<int> a(n);
  int tot = 0;
  for (int i = 0; i < n; ++i) {
    cin >> a[i];
    tot ^= a[i];
  }
  for (int i = 0; i < n; ++i)
    a[i] ^= tot;
  for (int i : a)
    cout << i << " ";
}