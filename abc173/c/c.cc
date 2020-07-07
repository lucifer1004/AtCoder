#include <iostream>
#include <vector>

using namespace std;
int main() {
  int h, w, k;
  cin >> h >> w >> k;
  vector<string> a(h);
  for (int i = 0; i < h; ++i)
    cin >> a[i];
  int ans = 0;
  for (int i = 0; i < (1 << h); ++i) {
    for (int j = 0; j < (1 << w); ++j) {
      vector<string> b(a);
      int cnt = 0;
      for (int p = 0; p < h; ++p)
        for (int q = 0; q < w; ++q) {
          if ((i & (1 << p)) || (j & (1 << q)))
            b[p][q] = '.';
          cnt += b[p][q] == '#';
        }
      ans += cnt == k;
    }
  }
  cout << ans;
}