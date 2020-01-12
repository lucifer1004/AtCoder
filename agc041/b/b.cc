#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;

int main() {
  ll n, m, v, p;
  cin >> n >> m >> v >> p;
  vector<ll> a(n), sum(n + 1);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  sort(a.begin(), a.end());
  reverse(a.begin(), a.end());
  for (int i = 0; i < n; ++i)
    sum[i + 1] = sum[i] + a[i];

  ll ans = p;
  for (int i = n - 1; i >= p; --i) {
    bool possible = false;
    ll maxa = a[i] + m;
    if (maxa < a[p - 1])
      continue;
    if (v <= p)
      possible = true;
    else {
      ll maxv =
          p * m + (n - i - 1) * m + maxa * (i - p + 1) - (sum[i] - sum[p - 1]);
      if (maxv >= v * m)
        possible = true;
    }
    if (possible) {
      ans = i + 1;
      break;
    }
  }
  cout << ans;
}