#include <stdint.h>
#include <stdio.h>

extern int32_t double_input(int32_t input);
extern int32_t third_input(int32_t input);

// gcc -o test_c main.c 2_3.lib
// ./test_c
int main() {
    int input = 4;
    int output = double_input(input);
    int output2 = third_input(input);
    printf("%d * 2 = %d\n", input, output);
    printf("%d * 3 = %d\n", input, output2);
    return 0;
}
