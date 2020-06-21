#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;
const ll MOD = 998244353;

int main() {
  string s;
  int k;
  cin >> s >> k;
  int one = 0, tot = 0;
  vector<int> cups;
  for (char c : s) {
    tot += c == '1';
    if (c == '0') {
      cups.emplace_back(one);
      one = 0;
    } else
      one++;
  }
  cups.emplace_back(one);
  int n = cups.size();
  k = min(k, tot);
  vector<vector<ll>> dp(tot + 1, vector<ll>(k + 1, -1));
  for (int i = 0; i <= k; ++i)
    dp[0][i] = 1;
  int acc = 0;
  for (int i = n - 1; i >= 0; --i) {
    acc += cups[i];
    vector<vector<ll>> ndp(tot + 1, vector<ll>(k + 1, -1));
    for (int j = 0; j <= acc; ++j) {
      if (j < cups[i]) {
        int delta = cups[i] - j;
        for (int p = 0; p + delta <= k; ++p) {
          for (int q = p; q + delta <= k; ++q) {
            if (dp[p][q] == -1)
              continue;
            if (ndp[p + delta][q + delta] == -1)
              ndp[p + delta][q + delta] = 0;
            ndp[p + delta][q + delta] += dp[p][q];
            ndp[p + delta][q + delta] %= MOD;
          }
        }
      } else {
        int delta = j - cups[i];
        for (int p = delta; p <= k; ++p) {
          for (int q = p; q <= k; ++q) {
            if (dp[p][q] == -1)
              continue;
            if (ndp[p - delta][q] == -1)
              ndp[p - delta][q] = 0;
            ndp[p - delta][q] += dp[p][q];
            ndp[p - delta][q] %= MOD;
          }
        }
      }
    }
    swap(dp, ndp);
  }
  cout << dp[0][k];
}