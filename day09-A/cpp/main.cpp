#include <bits/stdc++.h>

typedef long long ll;
typedef long double ld;
using namespace std;

const char ENDL = '\n';
const ll MOD = 1e9 + 7;

const int DIRS[8][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}, {1, -1}, {-1, 1}, {1, 1}, {-1, -1}};

int solve(const vector<string>& lines) {
    ll ans = 0;

    for (int i = 0; i < lines.size(); i++) {
        for (int j = 0; j < lines[0].size(); j++) {
            int count = 0;

            for (auto& dir: DIRS) {
                int x = i + dir[0], y = j + dir[1];
                count += x < lines.size() && x >= 0 && y < lines[0].size() && y >= 0 && lines[x][y] == '@';
            }

            //if (count < 4) cout << "[" << i << ", " << j << "];   ";

            ans += lines[i][j] == '@' && count < 4;
        }
    }


    return ans;
}

int main()
{
    cout << "STARTING" << ENDL;
    std::ifstream in("../input.txt");
    vector<string> lines;
    string line;
    while (std::getline(in, line))
    {
        lines.push_back(line);
    }
    cout << solve(lines) << ENDL;
    return 0;
}
