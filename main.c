#include <stdio.h>

#include "lib.h"

int main() {
    printf("Hello, world!\n");
    int a = 10;
    int b = 20;
    int c = add(a, b);
    printf("The sum of %d and %d is %d\n", a, b, c);
    int d = sub(a, b);
    printf("The difference of %d and %d is %d\n", a, b, d);
    return 0;
}
