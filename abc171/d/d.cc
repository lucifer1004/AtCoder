#include <iostream>
#include <map>
#include <vector>

using namespace std;
typedef long long ll;
int main() {
  map<int, int> cnt;
  int n;
  cin >> n;
  vector<int> a(n);
  ll sum = 0;
  for (int i = 0; i < n; ++i) {
    cin >> a[i];
    cnt[a[i]]++;
    sum += a[i];
  }
  int q;
  cin >> q;
  for (int i = 0; i < q; ++i) {
    int b, c;
    cin >> b >> c;
    if (!cnt.count(b)) {
      cout << sum << endl;
      continue;
    }
    sum += (ll)(c - b) * cnt[b];
    cnt[c] += cnt[b];
    cnt.erase(b);
    cout << sum << endl;
  }
}