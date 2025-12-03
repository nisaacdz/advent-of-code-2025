#include <bits/stdc++.h>

typedef long long ll;
typedef long double ld;
using namespace std;

const char ENDL = '\n';
const ll MOD = 1e9 + 7;

ll solve(const string& line) {
    string digits;

    for (int i = 0; i < line.size(); i++) {
        int current_digit = line[i];
        while (!digits.empty() && current_digit > digits.back() && (digits.size() - 1 + line.size() - i) >= 12) {
            digits.pop_back();
        }
        if (digits.size() < 12) digits.push_back(current_digit);
    }

    //cout << stoll(digits) << ENDL;

    return stoll(digits);
}

int main()
{
    cout << "STARTING" << ENDL;
    std::ifstream in("../input.txt");
    string line;
    ll sum_of_joltages = 0;
    while (std::getline(in, line))
    {
        sum_of_joltages += solve(line);
    }
    cout << sum_of_joltages << ENDL;
    return 0;
}
