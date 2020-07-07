#include <algorithm>
#include <iostream>
#include <set>
#include <vector>

using namespace std;
typedef long long ll;
int main() {
  int n;
  cin >> n;
  vector<int> a(n);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  sort(a.rbegin(), a.rend());
  ll ans = a[0];
  multiset<pair<int, int>, greater<>> s = {{a[1], a[0]}, {a[1], a[0]}};
  for (int i = 2; i < n; ++i) {
    auto it = s.begin();
    s.erase(it);
    ans += it->first;
    s.insert({a[i], it->first});
    s.insert({a[i], it->second});
  }
  cout << ans;
}