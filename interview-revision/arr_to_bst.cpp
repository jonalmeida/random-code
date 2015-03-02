#include <iostream>
#include <array>
#include <vector>
/*
 * Errors in the code:
 *  - It should use a vector instead of an array
 *  - Replace 'fill' with 'insert' to support vector
 */
using namespace std;

struct Node {
    Node *left;
    Node *right;
    int data;
};

std::array<int> split_arr(std::array<int> arr, int len) {
    std::array<int> ret;
    for(int i = 0; i < len; i++)
    {
        ret.fill(arr.at(i));
    }

    return ret;
}

void createBST(std::array<int> arr, Node *root) {
    if(arr.empty()){
        return;
    }
    int mid_pos = arr.size()/2;
    root->data = arr.at(mid_pos);
    //split array into two
    std::array<int> arr_left;
    int right_len = arr.size() - mid_pos - 1;
    for(int i = 0; i < (mid_pos - 1); i++) {
        arr_left.fill(arr.at(i));
    }
    for(int j = 0; j < right_len; i++) {
        arr_right.fill(arr.at(i+mid_pos);
    }

    std::array<int> arr_right;

    createBST(arr_left, root->left);
    createBST(arr_right, root->right);
}

int main() {
    return 0;
}


