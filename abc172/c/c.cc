#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
int main() {
  int n, m, k;
  cin >> n >> m >> k;
  vector<int> a(n), b(m);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  for (int i = 0; i < m; ++i)
    cin >> b[i];
  ll ans = 0, tot = 0, l = 0;
  while (tot <= k) {
    ans = l;
    if (l == n)
      break;
    tot += a[l++];
  }
  for (int r = 1; r <= m; ++r) {
    tot += b[r - 1];
    while (l >= 1 && tot > k)
      tot -= a[--l];
    if (tot > k)
      break;
    ans = max(ans, l + r);
  }
  cout << ans;
}