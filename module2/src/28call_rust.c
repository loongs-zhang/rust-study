// gcc -o call_rust 28call_rust.c libmodule2.a
// ./call_rust
#include<stdio.h>

extern int add(int a, int b);

int main() {
    printf("result->%d\n", add(5, 7));
    return 0;
}