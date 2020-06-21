#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
const ll mod = 998244353;

int main() {
  int a, b, c, d;
  cin >> a >> b >> c >> d;
  vector<vector<ll>> dp(c + 1, vector<ll>(d + 1));
  dp[a][b] = 1;
  for (int i = a + 1; i <= c; ++i)
    dp[i][b] = dp[i - 1][b] * b % mod;
  for (int j = b + 1; j <= d; ++j)
    dp[a][j] = dp[a][j - 1] * a % mod;
  for (int i = a + 1; i <= c; ++i)
    for (int j = b + 1; j <= d; ++j) {
      dp[i][j] = (dp[i - 1][j] * j + dp[i][j - 1] * i -
                  (i - 1) * (j - 1) * dp[i - 1][j - 1]) %
                 mod;
      if (dp[i][j] < 0)
        dp[i][j] += mod;
    }
  cout << dp[c][d] << endl;
}
