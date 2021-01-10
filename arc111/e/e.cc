#include <atcoder/math>
#include <iostream>

using namespace std;
using ll = long long;

int main() {
  int t;
  cin >> t;
  while (t--) {
    ll a, b, c, d;
    cin >> a >> b >> c >> d;
    ll hi = (d - 1) / (c - b);
    cout << hi + atcoder::floor_sum(hi + 1, d, b, a - 1) -
                atcoder::floor_sum(hi + 1, d, c, a)
         << endl;
  }
}