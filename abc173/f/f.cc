#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
int main() {
  int n;
  cin >> n;
  ll ans = 0;
  for (int i = 1; i <= n; ++i) {
    ll curr = (ll)i * (i + 1) / 2;
    ans += curr;
  }
  for (int i = 0; i < n - 1; ++i) {
    int u, v;
    cin >> u >> v;
    if (u > v)
      swap(u, v);
    ll left = u;
    ll right = n - v + 1;
    ans -= left * right;
  }
  cout << ans;
}