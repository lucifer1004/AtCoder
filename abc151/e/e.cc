#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;

const ll modulo = 1e9 + 7;

ll fast_exp(ll x, ll y) {
  if (y == 0) return 1;
  ll half = fast_exp(x, y / 2);
  ll res = half * half % modulo;
  if (y % 2 == 1) res = res * x % modulo;
  return res;
}

class Solution {
  vector<ll> fac, rev;

  ll C(int n, int k) {
    if (k > n) return 0;
    ll ans = fac[n] * rev[n - k] % modulo;
    ans = ans * rev[k] % modulo;
    return ans;
  }

 public:
  void solve() {
    int n, k;
    cin >> n >> k;
    vector<ll> a(n);
    for (int i = 0; i < n; ++i) cin >> a[i];
    if (k == 1) {
      cout << 0;
      return;
    }

    fac = vector<ll>(n + 1);
    rev = vector<ll>(n + 1);
    fac[0] = 1;
    for (int i = 1; i <= n; ++i) fac[i] = fac[i - 1] * i % modulo;
    for (int i = 0; i <= n; ++i) rev[i] = fast_exp(fac[i], modulo - 2);

    sort(a.begin(), a.end());
    ll lo = 0, hi = 0;
    for (int i = 0; i < n; ++i) {
      ll l = C(n - i - 1, k - 1) * a[i] % modulo;
      ll h = C(i, k - 1) * a[i] % modulo;
      lo = (lo + l) % modulo;
      hi = (hi + h) % modulo;
    }

    ll ans = (hi - lo) % modulo;
    ans = (ans + modulo) % modulo;

    cout << ans;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}