#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

using namespace std;
const double eps = 1e-8;

struct Point {
  int x, y;

  double dist(Point that) {
    return sqrt((x - that.x) * (x - that.x) + (y - that.y) * (y - that.y));
  }

  double cross(Point a, Point b) {
    return (a.x - x) * (b.y - y) - (a.y - y) * (b.x - x);
  }
};

Point p0;

bool isZero(double x) { return fabs(x) < eps; }

bool cmp(Point a, Point b) {
  double k = p0.cross(a, b);
  return !(k < 0 || (isZero(k) && p0.dist(a) < p0.dist(b)));
}

class Solution {
 public:
  void solve() {
    int n;
    cin >> n;
    vector<Point> p(n + 1);
    for (int i = 0; i < n; ++i) cin >> p[i].x >> p[i].y;
    if (n == 2) {
      printf("%.12lf", p[0].dist(p[1]) / 2);
      return;
    }

    int x = p[0].x;
    int y = p[0].y;
    int mi = 0;
    for (int i = 1; i < n; ++i) {
      if (p[i].x < x || (p[i].x == x && p[i].y < y)) {
        x = p[i].x;
        y = p[i].y;
        mi = i;
      }
    }
    swap(p[0], p[mi]);
    p0.x = p[0].x;
    p0.y = p[0].y;
    sort(p.begin() + 1, p.begin() + n - 1, cmp);
    p[n] = p[0];
    vector<Point> stack(n + 1);
    for (int i = 0; i < 3; ++i) {
      stack[i].x = p[i].x;
      stack[i].y = p[i].y;
    }
    int top = 2;
    for (int i = 3; i < n; ++i) {
      if (stack[top - 1].cross(stack[top], p[i]) < 0 && top >= 2) --top;
      stack[++top] = p[i];
    }

    double maxr = -1;
    double a, b, c, r, s;
    for (int i = 0; i < top; ++i) {
      for (int j = i + 1; j < top; ++j) {
        for (int k = j + 1; k <= top; ++k) {
          a = stack[i].dist(stack[j]);
          b = stack[i].dist(stack[k]);
          c = stack[j].dist(stack[k]);
          if (a * a + b * b < c * c || a * a + c * c < b * b ||
              b * b + c * c < a * a)
            r = max(max(a, b), c) / 2.0;
          else {
            s = fabs(stack[i].cross(stack[j], stack[k])) / 2.0;
            r = a * b * c / (s * 4.0);
          }
          if (maxr < r) maxr = r;
        }
      }
    }
    printf("%.12lf", maxr);
  }
};

int main() {
  Solution solution = Solution();
  solution.solve();
}
