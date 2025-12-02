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
    ll stops_at_zero = 0;
    int current_stop = 50;
    string line;
    while (std::getline(in, line))
    {
        int steps = stoi(line.substr(1));

        //cout << "about to jump " << line[0] << " by " << steps << " steps." << ENDL;

        if (line[0] == 'L') {
            current_stop = ((current_stop - steps) % 100 + 100) % 100;
        } else {
            current_stop = (current_stop + steps) % 100;
        }

        //cout << "currently at stop No." << current_stop << ENDL; 
        
        stops_at_zero += current_stop == 0;
    }
    cout << stops_at_zero << ENDL;
    return 0;
}
