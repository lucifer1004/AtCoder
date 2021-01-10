#include <atcoder/maxflow>
#include <iostream>
#include <vector>

using namespace std;

int main() {
  int n;
  cin >> n;
  int s = 0, t = n + 400001;
  atcoder::mf_graph<int> g(t + 1);
  for (int i = 1; i <= n; ++i) {
    int a, b;
    cin >> a >> b;
    g.add_edge(s, i, 1);
    g.add_edge(i, a + n, 1);
    g.add_edge(i, b + n, 1);
  }
  for (int i = 1; i <= 400000; ++i)
    g.add_edge(n + i, t, 1);
  cout << g.flow(s, t);
}