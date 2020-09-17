//
// Created by Fengda Huang  on 2020/9/17.
//
#ifndef UTF_STRING
#define UTF_STRING

typedef struct UtfString {
    int length;
    char *string;
} UtfString;

UtfString *newUtfString(char *str);
#endif /* UTF_STRING */
