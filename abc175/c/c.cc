#include <cstdio>
#include <iostream>

using namespace std;
typedef long long ll;

template <typename T> void read(T &x) {
  x = 0;
  char c = getchar();
  T sig = 1;
  for (; !isdigit(c); c = getchar())
    if (c == '-')
      sig = -1;
  for (; isdigit(c); c = getchar())
    x = (x << 3) + (x << 1) + c - '0';
  x *= sig;
}

class Solution {
public:
  void solve() {
    ll x, k, d;
    read(x), read(k), read(d);
    if (x < 0)
      x = -x;
    ll f = x / d;
    if (f >= k)
      printf("%lld", x - k * d);
    else {
      ll rem = x % d;
      if ((f - k) % 2 == 0)
        printf("%lld", rem);
      else
        printf("%lld", d - rem);
    }
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}