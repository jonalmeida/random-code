#include <iostream>
#include <vector>

using namespace std;

struct bst
{
    unsigned int data;
    int leftIdx;
    int rightIdx;
};

void MakeNode(vector<struct> &v1, int aData);
void setleft(vector <struct>&v1, int currIndex, int aData);
void setright(vector<struct> &v1, int currIndex, int aData);
void Insert(vector<struct bst> &v1, int aData);
