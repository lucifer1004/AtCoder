#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;

int main() {
  ll n, a, b;
  cin >> n >> a >> b;
  ll delta = abs(a - b);
  if (delta % 2 == 0) {
    cout << delta / 2;
  } else {
    ll p = min(a, b);
    ll q = max(a, b);
    ll ans = min(q - 1, n - p);
    ans = min(ans, p + (q - p - 1) / 2);
    ans = min(ans, n - q + 1 + (q - p - 1) / 2);
    cout << ans;
  }
}