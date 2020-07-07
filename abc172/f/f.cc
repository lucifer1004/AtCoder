#include <bitset>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
typedef bitset<64> bs;
int main() {
  int n;
  cin >> n;
  vector<ll> a(n);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  ll rest = 0;
  for (int i = 2; i < n; ++i)
    rest ^= a[i];
  ll sum = a[0] + a[1];
  bs x, y, z(rest);
  auto fail = []() {
    cout << -1;
    exit(0);
  };
  vector<int> v;
  ll xx = 0;
  for (int i = 60; i >= 0; --i) {
    if (!z[i]) {
      if (sum >= (1ll << (i + 1))) {
        x.set(i), y.set(i);
        sum -= (1ll << (i + 1));
        xx += (1ll << i);
      }
    } else {
      if (sum >= (1ll << i))
        v.emplace_back(i);
      else
        fail();
      sum -= (1ll << i);
    }
  }
  if (sum != 0)
    fail();
  for (int i : v) {
    if (xx + (1ll << i) < a[0]) {
      x.set(i);
      xx += (1ll << i);
    } else
      y.set(i);
  }
  if (xx > a[0] || xx == 0)
    fail();
  cout << a[0] - xx;
}