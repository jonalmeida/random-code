#include "vector_bst.hpp"

using namespace std;

void MakeNode(vector<struct> &v1, int aData)
{
    struct bst b1 = { aData, -1, -1 };
    v1.push_back(b1);
}

void setleft(vector <struct>&v1, int currIndex, int aData)
{
    unsigned int leftIndex = v1.size();
    v1[currIndex].leftIdx = leftIndex;
    struct bst b1 = { aData, -1, -1 };
    v1.push_back(b1);
}

void setright(vector<struct> &v1, int currIndex, int aData)
{
    unsigned int rightIndex = v1.size();
    v1[currIndex].rightIdx = rightIndex;
    struct bst b1 = { aData, -1, -1 };
    v1.push_back(b1);
}

void Insert(vector<struct bst> &v1, int aData)
{
    if(v1.size() == 0)
    {
        cout<<"Note is not made yet. MakeNode first..."<<endl;
        return;
    }
    unsigned int currentIdx = 0;
    while ( currentIdx < v1.size() )
    {
        if(aData <= v1[currentIdx].data)
        {
            if( v1[currentIdx].leftIdx == -1)
            {
                setleft(v1, currentIdx, aData);
                break;
            }
            else
                currentIdx = v1[currentIdx].leftIdx;
        }
        else
        {
            if(v1[currentIdx].rightIdx == -1)
            {
                setright(v1, currentIdx, aData);
                break;
            }
            else
                currentIdx = v1[currentIdx].rightIdx;
        }
    }
}

int main() {
    vector <bst> v1;
    MakeNode(v1, 30);
    Insert(v1, 20);
    Insert(v1, 6);
    Insert(v1, 40);
    Insert(v1, 35);
    Insert(v1, 16);
    Insert(v1, 7);

    return 0;
}
