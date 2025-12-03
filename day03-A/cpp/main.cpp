#include <bits/stdc++.h>

typedef long long ll;
typedef long double ld;
using namespace std;

const char ENDL = '\n';
const ll MOD = 1e9 + 7;

int solve(const string& line) {
    int tens_digit = 0, ones_digit = -1;

    for (int i = 0; i < line.size(); i++) {
        int current_digit = line[i] - '0';
        if (current_digit > tens_digit && (tens_digit == 0 || i + 1 < line.size())) {
            tens_digit = current_digit;
            ones_digit = -1;
        } else if (current_digit > ones_digit) {
            ones_digit = current_digit;
        }
    }

    return ones_digit == - 1 ? tens_digit : tens_digit * 10 + ones_digit;
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
