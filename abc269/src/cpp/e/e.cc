#include <iostream>

using namespace std;

class Solution {
  int query(int a, int b, int c, int d) {
    cout << "? " << a << " " << b << " " << c << " " << d << endl << flush;
    int num;
    cin >> num;
    return num;
  }

public:
  void solve() {
    int n;
    cin >> n;

    int lo = 1, hi = n;
    while (lo < hi) {
      int mid = (lo + hi) >> 1;
      int res = query(lo, mid, 1, n);
      if (res == mid - lo + 1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    int row = lo;

    lo = 1, hi = n;
    while (lo < hi) {
      int mid = (lo + hi) >> 1;
      int res = query(1, n, lo, mid);
      if (res == mid - lo + 1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    int col = lo;

    cout << "! " << row << " " << col << endl << flush;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}