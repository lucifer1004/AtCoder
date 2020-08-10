#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
ll fexp(ll x, ll y, ll m) {
  ll ans = 1;
  while (y) {
    if (y & 1)
      ans = ans * x % m;
    x = x * x % m;
    y >>= 1;
  }
  return ans;
}
int main() {
  int n;
  string x;
  cin >> n >> x;
  ll curr = 0;
  int tot = 0;
  for (char c : x)
    tot += c == '1';
  vector<ll> l(n), m(n), r(n);
  ll lt = 0, mt = 0, rt = 0;
  if (tot > 1) {
    l[0] = 1 % (tot - 1);
    for (int i = 1; i < n; ++i)
      l[i] = l[i - 1] * 2 % (tot - 1);
    for (int i = 0; i < n; ++i)
      if (x[i] == '1')
        lt = (lt + l[n - i - 1]) % (tot - 1);
  }
  if (tot > 0) {
    m[0] = 1 % tot;
    for (int i = 1; i < n; ++i)
      m[i] = m[i - 1] * 2 % tot;
    for (int i = 0; i < n; ++i)
      if (x[i] == '1')
        mt = (mt + m[n - i - 1]) % tot;
  }
  r[0] = 1 % (tot + 1);
  for (int i = 1; i < n; ++i)
    r[i] = r[i - 1] * 2 % (tot + 1);
  for (int i = 0; i < n; ++i)
    if (x[i] == '1')
      rt = (rt + r[n - i - 1]) % (tot + 1);
  string ans;
  for (int i = 0; i < n; ++i) {
    int curr = tot + (x[i] == '1' ? -1 : 1);
    if (curr <= 1) {
      ans += to_string(curr) + "\n";
      continue;
    }
    int t = 1;
    ll now;
    if (curr == tot + 1) {
      now = (rt + r[n - i - 1]) % (tot + 1);
    } else {
      now = (lt - l[n - i - 1]) % (tot - 1);
      if (now < 0)
        now += tot - 1;
    }
    while (now) {
      now %= __builtin_popcount(now);
      t++;
    }
    ans += to_string(t) + "\n";
  }
  cout << ans;
}