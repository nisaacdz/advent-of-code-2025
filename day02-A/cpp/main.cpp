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
    string line;
    regex re("(.+?)\\1");
    ll sum_of_bad_ids = 0;
    while (std::getline(in, line, ','))
    {
        ll seg = line.find('-');
        //cout << line << ENDL;
        ll rng_start = stoll(line.substr(0, seg));
        ll rng_end = stoll(line.substr(seg + 1));

        for (ll id = rng_start; id <= rng_end; id++) {
            if (regex_match(to_string(id), re)) {
                sum_of_bad_ids += id;
            }
        }
    }
    cout << sum_of_bad_ids << ENDL;
    return 0;
}
