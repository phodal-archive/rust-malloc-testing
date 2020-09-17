#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "utf_string.h"

typedef struct Scanner_ {
    int ptrs;
} Scanner;

Scanner *newScanner(char **patterns, int patternSize);

Scanner *newScanner(char **patterns, int patternSize) {
    for (int i = 0; i < patternSize; i++) {
        UtfString *utfString = newUtfString(patterns[i]);
        printf("%i", utfString->length);
    }

    void *scanner = malloc(sizeof(Scanner));
    return scanner;
}

int main() {
    char **strings = (char **) malloc(2 * sizeof(char *));
    strings[0] = "hello";
    strings[1] = "world";
    Scanner *pScanner = newScanner(strings, 2);

    UtfString *string = newUtfString("hello");
    printf("%s", string->string);
    printf("%i", string->length);
}

