#include <iostream>
using namespace std;

int fib(int);

int main()
{
    int sum = 0;
    int i = 0;
    int f = fib(i);
    while (f < 4000000)
    {
        cout << i << " " << f << '\n';
        if (f % 2 == 0)
            sum += f;
        i++;
        f = fib(i);
    }
    cout << sum << '\n';
    return 0;
}

int fib(int n)
{
    if (n <= 2)
        return (n % 2 + n) / 2; // 0->0, 1->1, 2->1
    return fib(n - 1) + fib(n - 2);
}