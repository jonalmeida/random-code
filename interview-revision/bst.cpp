#include <iostream>
#include <list>

class Node {
    public:
        int data;
        Node* left;
        Node* right;

        Node(int _data) {
            data = _data;
            left = NULL;
            right = NULL;
        }
};

class Tree {
    public:
        Node* head;

        Tree() {
            head = NULL;
        }

        void insert(Node _node) {
            Node *tmp = head;
            if(tmp == NULL)
            {
                head = &_node;
            } else {
                while(tmp != NULL)
                {
                    if(_node.data < tmp->data)
                    {
                    //    if(tmp->left == NULL)
                            tmp = tmp->left;
                    } else {
                    //    if(tmp->right == NULL)
                            tmp = tmp->right;
                    }
                }
                tmp = &_node;
            }
        }

        void print() {
            Node* tmp = head;
        }
};

int main() {
    Tree tree;
    tree.insert(Node(3));
    tree.insert(Node(2));
    tree.insert(Node(5));

    std::cout << tree.head->data << std::endl;
    return 0;
}
