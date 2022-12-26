//
//     ____                           ____       __    __    ______                __  
//    / __ \____ ____________      __/ __ \_____/ /___/ /   / ____/________ ______/ /__
//   / /_/ / __ `/ ___/ ___/ | /| / / / / / ___/ / __  /   / /   / ___/ __ `/ ___/ //_/
//  / ____/ /_/ (__  |__  )| |/ |/ / /_/ / /  / / /_/ /   / /___/ /  / /_/ / /__/ ,<   
// /_/    \__,_/____/____/ |__/|__/\____/_/  /_/\__,_/____\____/_/   \__,_/\___/_/|_|  
//                                                 /_____/                            
//
// ______________________________________________________________________________________ 
//| This is a rewritten version of the Rust language PassworldCrack project,             |
//|        because I found that the efficiency of the underlying language is really good,| 
//|        so I am going to rewrite it,                                                  |
//|        and the function may be similar.                                              |
//|                                                                                      |
//| 2022-12-26 <My emal>testsendkfotesycike@gmail.com                                    |
//|      Marry Christmas!                                                                |
//|                                                      <Make by GeumBo>                |
//|                                                                                      |
// --------------------------------------------------------------------------------------
//

#include <stdio.h>
#include <string.h>

#define CHARS "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\{}[]:;'<,>.?/"
#define CHARS_1 '"'

void _STRCAT(char *dest, const char *src)
{
    char str1[100] = src;
    char str2[100] = dest;
    char str3[200];
    for (i = 0; str1[i] != '\0'; ++i)
    {
        str3[i] = str1[i];
    }
    for (j = 0; str2[j] != '\0'; ++j)
    {
        str3[i + j] = str2[j];
    }
    str3[i + j] = '\0';
    return str3;
}

int main() {
char *strings = _STRCAT(CHARS, CHARS_1);
size_t len = strlen(strings);
for (int i = 1; i <= 5; i++) {
    for (int j = 0; j < len; j++) {
        printf("%c\n", CHARS[j]);
        for (int k = 0; k < len; k++) {
            printf("%c%c\n", CHARS[j], CHARS[k]);
            for (int l = 0; l < len; l++) {
                printf("%c%c%c\n", CHARS[j], CHARS[k], CHARS[l]);
                for (int m = 0; m < len; m++) {
                    printf("%c%c%c%c\n", CHARS[j], CHARS[k], CHARS[l], CHARS[m]);
                    for(int q =0; q < len; q++){
                        printf("%c%c%c%c%c\n", CHARS[j], CHARS[k], CHARS[l], CHARS[m],CHARS[q]);
                    }
                }
            }
        }
    }
}

return 0;
}