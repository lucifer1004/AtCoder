#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
int main() {
  int n;
  cin >> n;
  vector<int> f(n + 1, 2);
  f[1] = 1;
  ll ans = 1;
  for (int i = 2; i <= n; ++i) {
    ans += (ll)i * f[i];
    for (int j = 2 * i; j <= n; j += i)
      f[j]++;
  }
  cout << ans;
}