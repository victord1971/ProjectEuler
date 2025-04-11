
#include <stdio.h>
int main()
{
    char snum[18];
    long long T16, num;
    printf("Hello, World!\n");
    T16=0;
    for (num=11; num<1000; num++)
    {
        sprintf(snum,"%d",num);
        printf("%d  %s\n", num,snum);
    }

}


