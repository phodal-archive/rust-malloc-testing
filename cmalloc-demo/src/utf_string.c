//
// Created by Fengda Huang  on 2020/9/17.
//
#include "utf_string.h"
#include <stdlib.h>
#include <string.h>

UtfString *newUtfString(char *str) {
    UtfString *string;
    string = malloc(sizeof(UtfString));
    string->string = str;
    string->length = strlen(str);
    return string;
}
