#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;

class Solution {
 public:
  void solve() {
    int n, m;
    cin >> n >> m;
    vector<bool> ac(n + 1);
    vector<int> pen(n + 1);
    for (int i = 0; i < m; ++i) {
      int p;
      string s;
      cin >> p >> s;
      if (s == "AC")
        ac[p] = true;
      else {
        if (!ac[p]) pen[p]++;
      }
    }

    int s = 0, p = 0;
    for (int i = 1; i <= n; ++i) {
      s += ac[i];
      p += ac[i] * pen[i];
    }
    cout << s << " " << p;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}