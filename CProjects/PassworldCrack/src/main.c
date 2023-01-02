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


//#define chars "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\{}[]:;'<,>.?/"
char *chars[]={"a", "b", "c"};
int Crack(){
	size_t len = sizeof(chars);
	for (int i0=0; i0<=7; i0++)
{
	if(i0==1){
		for (int c0=0; c0<=len; c0++)
		{
			char passworld = chars[c0];
			printf("%s\n", passworld);sleep(0.6);
		}
	}
	
	if(i0==2)
	{
		for (int c0=0; c0<=len; c0++)
		{
			for (int c1=0; c1<=len; c1++)
			{
				printf("%s%s\n", chars[c0], chars[c1]);sleep(0.6);
			}
		}
	}
	/*if(i0==3)
	{
		for (int c0=0; c0<=len; c0++)
		{
			for (int c1=0; c1<=len; c1++)
			{
				for (int c2=0; c2<=len; c2++)
				{
					printf("%c%c%c/n", chars[c0], chars[c1], chars[c2]);sleep(0.6);
				}
			}
		}
	}if(i0==4)
	{
		for (int c0=0; c0<=len; c0++)
		{
			for (int c1=0; c1<=len; c1++)
			{
				for (int c2=0; c2<=len; c2++)
				{
					for (int c3=0; c3<=len; c3++)
					{
						printf("%c%c%c%c\n", chars[c0], chars[c1], chars[c2], chars[c3]);sleep(0.6);
					}
				}
			}
		}

}
*/
}
}

int main()
{
int len = sizeof(chars);
printf("%d\n", len);
sleep(2);
Crack();
}
