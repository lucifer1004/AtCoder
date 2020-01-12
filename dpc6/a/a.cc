#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  void solve() {
    int n;
    cin >> n;
    vector<string> songs(n);
    vector<int> durations(n);
    for (int i = 0; i < n; ++i) cin >> songs[i] >> durations[i];
    string that;
    cin >> that;
    int sum = 0;
    bool sleep = false;
    for (int i = 0; i < n; ++i) {
      if (sleep) sum += durations[i];
      if (that == songs[i]) sleep = true;
    }
    cout << sum;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}
