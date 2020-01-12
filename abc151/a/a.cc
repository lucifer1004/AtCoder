#include <iostream>

using namespace std;
typedef long long ll;

class Solution {
 public:
  void solve() {
    char c;
    cin >> c;
    cout << char(c + 1);
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}