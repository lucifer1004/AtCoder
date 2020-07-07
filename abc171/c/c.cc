#include <algorithm>
#include <iostream>

using namespace std;
typedef long long ll;

int main() {
  ll n;
  cin >> n;
  string ans;
  while (n) {
    n--;
    ans.push_back('a' + (n % 26));
    n /= 26;
  }
  reverse(ans.begin(), ans.end());
  cout << ans;
}