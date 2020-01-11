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
 public:
  void solve() {
    int n;
    cin >> n;
    vector<int> x(n);
    for (int i = 0; i < n; ++i) cin >> x[i];
    vector<int> dist(n);
    for (int i = 1; i < n; ++i) dist[i] = x[i] - x[i - 1];

    vector<ll> fac(n), rev(n), f(n);
    fac[0] = 1;
    fac[1] = 1;
    for (int i = 2; i < n; ++i) fac[i] = fac[i - 1] * i % modulo;
    for (int i = 1; i < n; ++i) rev[i] = fast_exp(fac[i], modulo - 2);
    f[0] = 0;
    for (int i = 1; i < n; ++i) {
      f[i] = (i * f[i - 1] + fac[i] - (i - 1) * fac[i - 1]) % modulo;
      f[i] = (f[i] + modulo) % modulo;
    }

    ll ans = 0;
    for (int i = 1; i < n; ++i) {
      ll coeff = f[i] * fac[n - 1] % modulo;
      coeff = coeff * rev[i] % modulo;
      ans = (ans + coeff * dist[i]) % modulo;
    }

    cout << ans;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}