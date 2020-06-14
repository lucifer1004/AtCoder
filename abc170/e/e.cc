#include <iostream>
#include <set>
#include <vector>
#define MAXN 200005

using namespace std;
int main() {
  int n, q;
  cin >> n >> q;
  vector<int> a(n), b(n);
  vector<set<pair<int, int>>> sc(MAXN);
  for (int i = 0; i < n; ++i) {
    cin >> a[i] >> b[i];
    sc[b[i]].insert({a[i], i});
  }
  set<pair<int, int>> s;
  for (int i = 1; i < MAXN; ++i)
    if (!sc[i].empty()) {
      s.insert({sc[i].rbegin()->first, i});
    }
  for (int i = 0; i < q; ++i) {
    int c, d;
    cin >> c >> d;
    c--;
    int idx = b[c];
    int hi = sc[idx].rbegin()->first;
    sc[idx].erase({a[c], c});
    if (sc[idx].empty() || sc[idx].rbegin()->first != hi) {
      s.erase({hi, idx});
      if (!sc[idx].empty())
        s.insert({sc[idx].rbegin()->first, idx});
    }
    if (b[c] != d) {
      hi = sc[d].rbegin()->first;
      sc[d].insert({a[c], c});
      if (sc[d].rbegin()->first != hi) {
        s.erase({hi, d});
        s.insert({sc[d].rbegin()->first, d});
      }
    }
    cout << s.begin()->first << endl;
    b[c] = d;
  }
}