#include <iostream>
#include <vector>
#define N 2000005

using namespace std;
typedef long long ll;
const ll MOD = 1e9 + 7;
ll fac[N], rev[N];
ll fexp(ll x, ll y) {
  ll ans = 1;
  while (y) {
    if (y & 1)
      ans = ans * x % MOD;
    x = x * x % MOD;
    y >>= 1;
  }
  return ans;
}
ll C(ll n, ll k) {
  ll ans = fac[n] * rev[k] % MOD;
  ans = ans * rev[n - k] % MOD;
  return ans;
}
int main() {
  int k;
  string s;
  cin >> k >> s;
  int n = s.size();
  fac[0] = 1;
  rev[0] = 1;
  for (int i = 1; i <= n + k; ++i) {
    fac[i] = fac[i - 1] * i % MOD;
    rev[i] = fexp(fac[i], MOD - 2);
  }
  ll tot = fexp(26, n + k);
  ll bad = 0;
  for (int i = 0; i < n; ++i) {
    bad = (bad + fexp(25, n + k - i) * C(n + k, i) % MOD) % MOD;
  }
  tot = (tot - bad + MOD) % MOD;
  cout << tot;
}