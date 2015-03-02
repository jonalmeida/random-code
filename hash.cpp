#include <iostream>
#include <string>
#include <inttypes.h>

int main( int argc, char *argv[] ) {
    uint64_t h = 7;
    std::string letters = "acdegilmnoprstuw";
    std::string s = argv[1];
    for(uint32_t i = 0; i < s.length(); i++) {
        std::cout << "letters.find(s[i]): " << letters.find(s[i]);
        std::cout << "\ts[i]: " << s[i];
        std::cout << "\th: " << h << std::endl;
        h = (h * 37 + letters.find(s[i]));
    }
    std::cout << s << " " << h << std::endl;
}
