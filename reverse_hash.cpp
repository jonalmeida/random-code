#include <iostream>
#include <string>
#include <inttypes.h>

int main( int argc, char *argv[] ) {

    uint64_t h = 25180466553932;
    std::string letters = "acdegilmnoprstuw";
    std::string answer = "";

    while (h != 7) {
        int leftover = h % 37;
        h = (h - leftover) / 37;
        answer.insert(0, sizeof(char), letters[leftover]);
        std::cout << " h: " << h << std::endl;
    }

    std::cout << answer << std::endl;

}
