#include <iostream>
#include <queue>
#include <set>
#include <vector>

using namespace std;
int main() {
  int h, w, k;
  cin >> h >> w >> k;
  int x1, y1, x2, y2;
  cin >> x1 >> y1 >> x2 >> y2;
  x1--, y1--, x2--, y2--;
  vector<string> mat(h);
  for (int i = 0; i < h; ++i)
    cin >> mat[i];
  vector<vector<int>> dist(h, vector<int>(w, -1));
  vector<set<int>> row(h), col(w);
  vector<vector<int>> llim(h, vector<int>(w)), rlim(h, vector<int>(w)),
      ulim(h, vector<int>(w)), dlim(h, vector<int>(w));
  for (int i = 0; i < h; ++i)
    for (int j = 0; j < w; ++j) {
      if (mat[i][j] == '@' || (i == x1 && j == y1))
        continue;
      row[i].insert(j);
      col[j].insert(i);
    }
  for (int i = 0; i < h; ++i) {
    int l = -1;
    for (int j = 0; j < w; ++j) {
      llim[i][j] = l;
      if (mat[i][j] == '@')
        l = j;
    }
    int r = w;
    for (int j = w - 1; j >= 0; --j) {
      rlim[i][j] = r;
      if (mat[i][j] == '@')
        r = j;
    }
  }
  for (int j = 0; j < w; ++j) {
    int u = -1;
    for (int i = 0; i < h; ++i) {
      ulim[i][j] = u;
      if (mat[i][j] == '@')
        u = i;
    }
    int d = h;
    for (int i = h - 1; i >= 0; --i) {
      dlim[i][j] = d;
      if (mat[i][j] == '@')
        d = i;
    }
  }
  queue<pair<int, int>> q;
  q.push({x1, y1});
  dist[x1][y1] = 0;
  while (!q.empty()) {
    auto [i, j] = q.front();
    q.pop();
    vector<pair<int, int>> nxt;

    // row
    if (!row[i].empty()) {
      auto it = row[i].upper_bound(j);
      for (auto it1 = it; it1 != row[i].end(); ++it1) {
        if (*it1 - j > k || *it1 >= rlim[i][j])
          break;
        nxt.emplace_back(i, *it1);
      }
      if (it != row[i].begin()) {
        it--;
        for (auto it2 = it; it2 != row[i].begin(); --it2) {
          if (j - *it2 > k || *it2 <= llim[i][j])
            break;
          nxt.emplace_back(i, *it2);
        }
        if (j - *row[i].begin() <= k && *row[i].begin() > llim[i][j])
          nxt.emplace_back(i, *row[i].begin());
      }
    }

    // col
    if (!col[j].empty()) {
      auto it = col[j].upper_bound(i);
      for (auto it1 = it; it1 != col[j].end(); ++it1) {
        if (*it1 - i > k || *it1 >= dlim[i][j])
          break;
        nxt.emplace_back(*it1, j);
      }
      if (it != col[j].begin()) {
        it--;
        for (auto it2 = it; it2 != col[j].begin(); --it2) {
          if (i - *it2 > k || *it2 <= ulim[i][j])
            break;
          nxt.emplace_back(*it2, j);
        }
        if (i - *col[j].begin() <= k && *col[j].begin() > ulim[i][j])
          nxt.emplace_back(*col[j].begin(), j);
      }
    }

    for (auto [ci, cj] : nxt) {
      dist[ci][cj] = dist[i][j] + 1;
      if (ci == x2 && cj == y2) {
        cout << dist[ci][cj];
        exit(0);
      }
      q.push({ci, cj});
      row[ci].erase(cj);
      col[cj].erase(ci);
    }
  }
  cout << -1;
}