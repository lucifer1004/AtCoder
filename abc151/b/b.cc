#include <iostream>

using namespace std;
typedef long long ll;

class Solution {
 public:
  void solve() {
    int n, k, m;
    cin >> n >> k >> m;
    int sum = 0;
    for (int i = 0; i < n - 1; ++i) {
      int a;
      cin >> a;
      sum += a;
    }
    if (sum + k >= m * n)
      cout << max(0, m * n - sum);
    else
      cout << -1;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}