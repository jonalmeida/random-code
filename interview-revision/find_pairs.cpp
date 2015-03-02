#include <iostream>
#include <array>

std::array<int, 8> arrayA = {1,3,7,9};

std::array<int, 4> arrayB= {2,4,5,6};

void merge_arrays(std::array<int, 8> arrayA, std::array<int,4> arrayB, int last_pos_A, int last_pos_B) {
    int last_A = last_pos_A - 1;
    int last_B = last_pos_B - 1;
    int end_of_array = last_pos_A + last_pos_B - 1;

    std::cout << "last_A: " << last_A << std::endl;
    std::cout << "last_B: " << last_B << std::endl;
    std::cout << "end_of_array: " << end_of_array << std::endl;

    //while(last_A >= 0 && last_B >= 0) {
        std::cout << "arrayA[" << last_A << "]: " << arrayA[last_A] << std::endl;
        if(arrayA[last_A] > arrayB[last_B]) {
            std::cout << "HERE!" << std::endl;
            arrayA[end_of_array] = arrayA[last_A];
            end_of_array--;
            last_A--;
        } else {
            arrayA[end_of_array] = arrayB[last_B];
            end_of_array--;
            last_B;
        }
    //}

    //while(last_B >= 0) {
    //    arrayA[end_of_array] = arrayB[last_B];
    //    last_B--;
    //}
}

int main() {

    merge_arrays(arrayA, arrayB, 4, 4);

    for(int i : arrayA) {
        std::cout << i << std::endl;
    }


    return 0;
}
