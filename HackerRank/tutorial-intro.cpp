#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;


int main() {

    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    int v, n;
    cin >> v >> n;
    int j[n];
    for(int i = 0; i < n; i++)
    {
        cin >> j[i];
    }

    for(int k = 0; k < n; ++k)
    {
        if(j[k] == v)
        {
            cout << k;
            break;
        }
    }

    //cout << endl;
    return 0;
}

