#include <iostream>

using namespace std;
const int MAX = 1e9;

class Solution {
 public:
  void solve() {
    int n, k, s;
    cin >> n >> k >> s;
    for (int i = 0; i < k; ++i) cout << s << " ";
    for (int i = k; i < n; ++i) cout << (s == MAX ? 1 : s + 1) << " ";
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}