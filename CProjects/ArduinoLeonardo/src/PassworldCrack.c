#include "../arduinoLib/libraries/Keyboard/src/Keyboard.h"
#include "../arduinoLib/libraries/Keyboard/src/KeyboardLayout.h"
//
//      __  __________     __  _____   ________ __ _____   ________   ___  _______________   ________ __
//     / / / /  _/ __ \   / / / /   | / ____/ //_//  _/ | / / ____/  /   |/_  __/_  __/   | / ____/ //_/
//    / /_/ // // / / /  / /_/ / /| |/ /   / ,<   / //  |/ / / __   / /| | / /   / / / /| |/ /   / ,<   
//   / __  // // /_/ /  / __  / ___ / /___/ /| |_/ // /|  / /_/ /  / ___ |/ /   / / / ___ / /___/ /| |  
//  /_/ /_/___/_____/  /_/ /_/_/  |_\____/_/ |_/___/_/ |_/\____/  /_/  |_/_/   /_/ /_/  |_\____/_/ |_|  
//                                                                                                   
// _____________________________________________________________________________________________
//| Using the HID (Human Interface Devie) vulnerability of the USB interface                    |
//|                                                                                             |
//| you can operate the computer to execute the content by simulating the keyboard or mouse     |
//| without obtaining special advanced permissions!                                             |
//|                                                                                             |
//| 2022-12-26 12:07PM <My emal>testsendkfotesycike@gmail.com                                   |
//| 	Merry Christmas!                                                                        |
//|                                                              <Make by GeumBo>               |
// ---------------------------------------------------------------------------------------------
//



#define p Keybord.println


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
void _STRLEN(char *strings)
{
    char str[] = strings;
    int length = 0;
    while (str[length] != '\0')
    {
        ++length;
    }
    return length;
}
void setup() {
	char strings = _STRCAT("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{\\}[]:;'<,>.?/",'"');
    char len = _STRLEN(strings)
    
        
}

void loop() {
}
