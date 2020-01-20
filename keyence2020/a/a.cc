#include <iostream>

using namespace std;

class Solution {
 public:
  void solve() {
    int h, w, n;
    cin >> h >> w >> n;
    cout << (n - 1) / max(h, w) + 1;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}