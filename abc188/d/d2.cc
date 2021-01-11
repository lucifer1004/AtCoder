#include <iostream>
#include <map>
#include <set>
#include <vector>

using namespace std;
typedef long long ll;

int main() {
  int N;
  ll C;
  cin >> N >> C;
  vector<int> a(N), b(N), c(N);
  set<int> s;
  map<int, vector<int>> starts, ends;
  for (int i = 0; i < N; ++i) {
    cin >> a[i] >> b[i] >> c[i];

    // We only need a[i] and b[i]+1 to represent the final segments.
    // For example, [1, 4] and [3, 8] will make
    // [1, 2], [3, 4], [5, 8] and [9, +inf].
    // We need 1, 3, 5, and 9 to represent these segments.
    s.insert(a[i]), s.insert(b[i] + 1);

    // We use two maps to store the start and end of a service.
    starts[a[i]].emplace_back(i);
    ends[b[i] + 1].emplace_back(i);
  }

  vector<int> v(s.begin(), s.end());
  int M = v.size();

  ll ans = 0, acc = 0;
  for (int i = 0; i < M - 1; ++i) {
    // These services have ended the day before.
    if (ends.count(v[i])) {
      for (int j : ends[v[i]])
        acc -= c[j];
    }

    // These services start on the current day.
    if (starts.count(v[i])) {
      for (int j : starts[v[i]])
        acc += c[j];
    }

    // Add to the total cost.
    ans += min(C, acc) * (v[i + 1] - v[i]);
  }
  cout << ans;
}
