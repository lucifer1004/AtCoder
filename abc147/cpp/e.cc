#include <bitset>
#include <iostream>
#include <vector>

/// In this solution, we use bitset to accelerate state transitions.

using namespace std;
using bs = bitset<25600>;
const int OFFSET = 12800;
int main() {
    int h, w;
    cin >> h >> w;
    vector<vector<int>> a(h, vector<int>(w));
    vector<vector<int>> b(a);
    for (int i = 0; i < h; ++i)
        for (int j = 0; j < w; ++j)
            cin >> a[i][j];
    for (int i = 0; i < h; ++i)
        for (int j = 0; j < w; ++j)
            cin >> b[i][j];
    vector<bs> last(w);
    last[0].set(OFFSET);

    for (int i = 0; i < h; ++i) {
        vector<bs> now(w);
        for (int j = 0; j < w; ++j) {
            int delta = abs(a[i][j] - b[i][j]);
            now[j] |= (last[j] << delta);
            now[j] |= (last[j] >> delta);
            if (j > 0) {
                now[j] |= (now[j - 1] << delta);
                now[j] |= (now[j - 1] >> delta);
            }
        }
        last = move(now);
    }

    int ans = OFFSET;
    for (int i = 0; i < 25600; ++i)
        if (last[w - 1][i])
            ans = min(ans, abs(i - OFFSET));

    cout << ans;
}