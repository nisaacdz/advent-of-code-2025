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
        steps_thru_zero += total_steps / 100;

        ll steps = total_steps % 100;

        if (line[0] == 'L') {
            steps_thru_zero += current_stop && (current_stop - steps) <= 0;
            current_stop = ((current_stop - steps) % 100 + 100) % 100;
        } else {
            steps_thru_zero += (current_stop + steps) > 99;
            current_stop = (current_stop + steps) % 100;
        }
    }
    cout << steps_thru_zero << ENDL;
    return 0;
}
