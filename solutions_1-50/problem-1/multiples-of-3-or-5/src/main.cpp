#include <iostream>
using namespace std;

int main()
{
    int sum = 0;
    for (int i = 0; i < 1000; ++i)
    {
        sum += (i % 3) == 0 || (i % 5) == 0 ? i : 0;
    }
    cout << sum << '\n';
    return 0;
}