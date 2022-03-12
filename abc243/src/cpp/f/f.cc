#include <atcoder/modint>
#include <cstdio>
#include <iostream>
#include <vector>

using namespace std;
using mint = atcoder::modint998244353;

template <typename T>
void read(T &x) {
  x = 0;
  char c = getchar();
  T sig = 1;
  for (; !isdigit(c); c = getchar())
    if (c == '-') sig = -1;
  for (; isdigit(c); c = getchar()) x = (x << 3) + (x << 1) + c - '0';
  x *= sig;
}

const int N = 200;

class Solution {
 public:
  void solve() {
    int n, m, k;
    read(n), read(m), read(k);
    vector<int> w(n);
    int sw = 0;
    for (int i = 0; i < n; i++) {
      read(w[i]);
      sw += w[i];
    }
    if (n == 1) {
      printf("1\n");
      return;
    }

    vector<mint> p(n);
    for (int i = 0; i < n; i++) p[i] = mint(w[i]) / sw;

    vector<vector<mint>> C(N, vector<mint>(N));
    C[0][0] = 1;
    for (int i = 1; i < N; i++) {
      C[i][0] = C[i][i] = 1;
      for (int j = 1; j < i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
    }

    vector<vector<mint>> P(k + 1, vector<mint>(n + 1));
    P[0][0] = 1;

    for (int i = 0; i < n; i++) {
      for (int t = k - 1; t >= 0; t--) {
        for (int j = n - 1; j >= 0; j--) {
          if (P[t][j] == 0) continue;
          mint pi(1);
          for (int cnt = 1; cnt + t <= k; cnt++) {
            pi *= p[i];
            P[cnt + t][j + 1] += P[t][j] * pi * C[cnt + t][t];
          }
        }
      }
    }

    printf("%d\n", P[k][m].val());
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}