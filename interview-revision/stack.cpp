#include <iostream>
#include <stack>

class Node {
    public:
        int data;
        int min;
        Node(int _data) {
            data = _data;
            min = -1;
        }
};

class Stack {
    public:
        Stack(){ };

        void pop() {
            m_stack.pop();
        }

        void push(Node _new) {
            if(m_stack.empty())
            {
                _new.min = _new.data;
            }
            else
            {
                if(m_stack.top().min > _new.data)
                {
                    _new.min = _new.data;
                }
                else
                {
                    _new.min = m_stack.top().min;
                }
             }
            m_stack.push(_new);
        }

        int min() {
            return m_stack.top().min;
        }
    private:
        std::stack<Node> m_stack;
};

int main() {
    Stack mystack;
    mystack.push(Node(3));
    mystack.push(Node(2));
    mystack.push(Node(5));
    mystack.push(Node(1));
    mystack.pop();
    std::cout << mystack.min() << std::endl;

    return 0;
}
