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
#include <unistd.h>


#define CHARS "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\{}[]:;'<,>.?/"
#define CHARS_1 '"'

int main() {
//char *strings = _STRCAT(CHARS, CHARS_1);
size_t len = strlen(CHARS);
for (int i0=0; i0<=7; i0++)
{
	if(i0==1){
		for (int c0=0; c0<len; c0++)
		{
			printf("%c\n", CHARS[c0]);sleep(0.6);
		}
	}if(i0==2)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				printf("%c%c\n", CHARS[c0], CHARS[c1]);sleep(0.6);
			}
		}
	}if(i0==3)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					printf("%c%c%c/n", CHARS[c0], CHARS[c1], CHARS[c2]);sleep(0.6);
				}
			}
		}
	}if(i0==4)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					for (int c3=0; c3<len; c3++)
					{
						printf("%c%c%c%c\n", CHARS[c0], CHARS[c1], CHARS[c2], CHARS[c3]);sleep(0.6);
					}
				}
			}
		}
	}if(i0==4)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					for (int c3=0; c3<len; c3++)
					{
						printf("%c%c%c%c\n", CHARS[c0], CHARS[c1], CHARS[c2], CHARS[c3]);sleep(0.6);
					}
				}
			}
		}
	}if(i0==5)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					for (int c3=0; c3<len; c3++)
					{
						for (int c4=0; c4<len; c4++)
						{
							printf("%c%c%c%c%c\n", CHARS[c0], CHARS[c1], CHARS[c2], CHARS[c3], CHARS[c4]);sleep(0.6);
						}
					}
				}
			}
		}
	}if(i0==6)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					for (int c3=0; c3<len; c3++)
					{
						for (int c4=0; c4<len; c4++)
						{
							for (int c5=0; c5<len; c5++)
							{
								printf("%c%c%c%c%c%c\n", CHARS[c0], CHARS[c1], CHARS[c2], CHARS[c3], CHARS[c4],CHARS[c5]);sleep(0.6);
							}
						}
					}
				}
			}
		}
	}if(i0==7)
	{
		for (int c0=0; c0<len; c0++)
		{
			for (int c1=0; c1<len; c1++)
			{
				for (int c2=0; c2<len; c2++)
				{
					for (int c3=0; c3<len; c3++)
					{
						for (int c4=0; c4<len; c4++)
						{
							for (int c5=0; c5<len; c5++)
							{
								for (int c6=0; c6<len; c6++)
								{
									printf("%c%c%c%c%c%c\n", CHARS[c0], CHARS[c1], CHARS[c2], CHARS[c3], CHARS[c4],CHARS[c5],CHARS[c6]);sleep(0.6);
								}
							}
						}
					}
				}
			}
		}
	}
}
}
