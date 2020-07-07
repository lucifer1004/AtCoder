#include <iostream>
#include <vector>
#define MAXN 500005

using namespace std;
typedef long long ll;
const ll MOD = 1e9 + 7;
ll fac[MAXN], rev[MAXN];

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

ll A(ll n, ll k) { return fac[n] * rev[n - k] % MOD; }

ll C(ll n, ll k) { return A(n, k) * rev[k] % MOD; }

int main() {
  int n, m;
  cin >> n >> m;
  fac[0] = 1, rev[0] = 1;
  for (int i = 1; i <= m; ++i) {
    fac[i] = fac[i - 1] * i % MOD;
    rev[i] = fexp(fac[i], MOD - 2);
  }
  ll tot = A(m, n) * A(m, n) % MOD;
  int sign = 1;
  for (int i = 1; i <= n; ++i) {
    sign = -sign;
    ll now = C(n, i) * A(m, i) % MOD;
    now = now * A(m - i, n - i) % MOD;
    now = now * A(m - i, n - i) % MOD;
    now = now * sign % MOD;
    if (now < 0)
      now += MOD;
    tot = (tot + now) % MOD;
  }
  cout << tot;
}