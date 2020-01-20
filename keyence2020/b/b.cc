#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

struct Segment {
  int l, r;
};

class Solution {
 public:
  void solve() {
    int n;
    cin >> n;
    vector<Segment> s;
    for (int i = 0; i < n; ++i) {
      int x, l;
      cin >> x >> l;
      s.push_back({x - l, x + l});
    }

    sort(s.begin(), s.end(), [](Segment a, Segment b) {
      return a.l < b.l || (a.l == b.l && a.r < b.r);
    });

    int ans = 0, l = s[0].l, r = s[0].r;
    for (int i = 1; i < n; ++i) {
      if (s[i].l >= r) {
        ans++;
        l = s[i].l;
        r = s[i].r;
      } else if (s[i].r <= r) {
        l = s[i].l;
        r = s[i].r;
      }
    }
    ans++;
    cout << ans;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}