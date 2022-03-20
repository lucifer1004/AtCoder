#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<bool> used(n * 2 + 2);
    cout << 1 << endl;
    used[1] = true;
    int ptr = 2;

    for (int i = 1; i <= n; ++i) {
        int x;
        cin >> x;
        used[x] = true;

        while (used[ptr])
            ptr++;
        cout << ptr << endl;
        used[ptr] = true;
    }

    int result;
    cin >> result;
    assert(result == 0);
}
