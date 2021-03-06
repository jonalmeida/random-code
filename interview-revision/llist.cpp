#include <iostream>

class Node {
    public:
    int data;
    Node* next = NULL;

    Node () { }
    void setData(int d) { data = d; }
    void setNext(Node* n) { next = n; }

};

class List {
    public:
    Node* head;
    List() { head = NULL; }
    void append(int d)
    {
        Node* tmp = head;

        Node* newNode = new Node();
        newNode->setData(d);
        newNode->setNext(NULL);

        if(tmp != NULL) {
            while (tmp->next != NULL) {
                tmp = tmp->next;
            }
            tmp->setNext(newNode);
        }
        else {
            head = newNode;
        }
    }
};

int main() {
    List* myList = new List();
    myList->append(5);
    myList->append(1);
    myList->append(2);
    myList->append(3);
    myList->append(4);

    Node* listPtr = myList->head;

    while(listPtr != NULL) {
        std::cout << listPtr->data << std::endl;
        listPtr = listPtr->next;
    }
}
