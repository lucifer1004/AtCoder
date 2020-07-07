#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
const ll MOD = 1e9 + 7;
int main() {
  ll n, k;
  cin >> n >> k;
  vector<ll> pos, neg;
  vector<pair<ll, ll>> p;
  ll zero = 0;
  for (ll i = 0; i < n; ++i) {
    ll a;
    cin >> a;
    p.emplace_back(abs(a), a > 0 ? 1 : (a == 0 ? 0 : -1));
    if (a == 0)
      zero++;
    else if (a > 0)
      pos.emplace_back(a);
    else
      neg.emplace_back(a);
  }
  ll np = pos.size(), nn = neg.size();
  if (k > np + nn) {
    cout << 0;
    return 0;
  }
  sort(neg.rbegin(), neg.rend());
  sort(p.rbegin(), p.rend());
  if (nn == n && k % 2 == 1) {
    ll ans = 1;
    for (ll i = 0; i < k; ++i)
      ans = ans * neg[i] % MOD;
    if (ans < 0)
      ans += MOD;
    cout << ans;
    return 0;
  }

  ll bal = 0;
  for (ll i = 0; i < k; ++i)
    bal += p[i].second == -1;
  if (bal % 2 == 0) {
    ll ans = 1;
    for (ll i = 0; i < k; ++i)
      ans = ans * p[i].first % MOD;
    if (ans < 0)
      ans += MOD;
    cout << ans;
    return 0;
  }

  ll x1 = -1, y1 = -1, x2 = -1, y2 = -1;
  // - => +
  {
    ll x = k - 1;
    while (x >= 0 && p[x].second == 1)
      x--;
    ll y = k;
    while (y < n && p[y].second == -1)
      y++;
    if (x >= 0 && y < n)
      x1 = x, y1 = y;
  }

  // + => -
  {
    ll x = k - 1;
    while (x >= 0 && p[x].second == -1)
      x--;
    ll y = k;
    while (y < n && p[y].second == 1)
      y++;
    if (x >= 0 && y < n)
      x2 = x, y2 = y;
  }

  if (x1 == -1) {
    swap(p[x2], p[y2]);
  } else if (x2 == -1) {
    swap(p[x1], p[y1]);
  } else {
    ll left = (ll)p[x1].first * p[y2].first;
    ll right = (ll)p[x2].first * p[y1].first;
    if (left < right)
      swap(p[x1], p[y1]);
    else
      swap(p[x2], p[y2]);
  }
  ll ans = 1;
  for (ll i = 0; i < k; ++i)
    ans = ans * p[i].first % MOD;
  if (ans < 0)
    ans += MOD;
  cout << ans;
}
