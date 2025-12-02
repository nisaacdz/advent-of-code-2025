#include <bits/stdc++.h>

typedef long long ll;
typedef long double ld;
using namespace std;

const char ENDL = '\n';
const ll MOD = 1e9 + 7;

int main()
{
    cout << "STARTING" << ENDL;
    std::ifstream in("../input.txt");
    ll steps_thru_zero = 0;
    ll current_stop = 50;
    string line;
    while (std::getline(in, line))
    {
        ll total_steps = stoi(line.substr(1));

        if (line[0] == 'L') {
            for (int i = 0; i < total_steps; i++) {
                current_stop = ((current_stop - 1) % 100 + 100) % 100;
                steps_thru_zero += current_stop == 0;
            }
        } else {
            for (int i = 0; i < total_steps; i++) {
                current_stop = (current_stop + 1) % 100;
                steps_thru_zero += current_stop == 0;
            }
        }
    }
    cout << steps_thru_zero << ENDL;
    return 0;
}
