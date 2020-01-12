#include <iostream>
#include <queue>
#include <vector>

using namespace std;
typedef long long ll;

const int dx[4] = {-1, 0, 1, 0};
const int dy[4] = {0, -1, 0, 1};

struct Node {
  int x, y, steps;
};

class Solution {
 public:
  void solve() {
    int n, m;
    cin >> n >> m;
    vector<string> maze(n);
    for (int i = 0; i < n; ++i) cin >> maze[i];
    int ans = 0;
    for (int i = 0; i < n; ++i)
      for (int j = 0; j < m; ++j) {
        if (maze[i][j] == '#') continue;
        queue<Node> q;
        vector<vector<bool>> visited(n, vector<bool>(m));
        visited[i][j] = true;
        q.push(Node{j, i, 0});
        while (!q.empty()) {
          Node f = q.front();
          q.pop();
          ans = max(ans, f.steps);
          for (int k = 0; k < 4; ++k) {
            int nx = f.x + dx[k];
            int ny = f.y + dy[k];
            if (nx < 0 || nx >= m || ny < 0 || ny >= n || visited[ny][nx] ||
                maze[ny][nx] == '#')
              continue;
            visited[ny][nx] = true;
            q.push(Node{nx, ny, f.steps + 1});
          }
        }
      }
    cout << ans;
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}