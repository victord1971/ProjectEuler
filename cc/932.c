
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int substr(char *src, char *dest, int num, int len) {
    /* проверка случая 4 */
    if ( (num<0)||(len<=0) ) return dest[0]=0;
    /* выход на num-ый символ или на конец строки */
    while ( num-- && *src++ );
    /* проверка случая 3 */
    if ( !num ) return dest[0]=0;
    /* перезапись символов */
    while ( len-- && *src ) *dest++=*src++;
    /* запись признака конца в выходную строку */
    *dest=0;
    return 1;
   }

int main()
{
    char snum[18], te1[17], te2[17];
    int i, len;
    long long T16, num, te1d, te2d, temp;
    T16=0;
                    //1234567890123456
    for (num=11; num<10000; num++)
    {
        sprintf(snum,"%d",num);
        //printf("%d  %s  ", num, snum);
        i=0;
        while (snum[i]!='\0')
        {
            //strncpy(destination, source, 10);
            len=strlen(snum);
            if((i<len-1) && (snum[i+1]!='0'))
            {
                substr(snum,te1,0,i+1); //te2=substr(snum,i+1,len-i);
                //printf("%s  %d  %d  %s    ",snum,i,len,te1);
                te1d=atoll(te1);
                substr(snum,te2,i+1,len-i);
                te2d=atoll(te2);
                //printf("%s  %d  %s %d    ",snum,len,te1,te2d);
                temp=te1d+te2d;
                if(temp*temp==num)
                {
                    T16+=num;
                    printf("%d    %d %d\n", T16,te1d,te2d);
                }
            }
            i++;
        }
        //printf("\n");
    }
    printf("%lld\n", T16);
}
