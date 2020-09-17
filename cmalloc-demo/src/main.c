#include <stdio.h>
#include "utf_string.h"

int main() {
    UtfString *string = newUtfString("hello");

    printf("%s", string->string);
    printf("%i", string->length);
}

