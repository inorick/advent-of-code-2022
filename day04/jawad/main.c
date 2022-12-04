#include <stdint.h>
#include <stdio.h>

typedef uint32_t u32;

int main(void)
{
    char line[64];
    u32 first_start = 0;
    u32 first_end = 0;
    u32 second_start = 0;
    u32 second_end = 0;
    u32 pair_counter = 0;
    while(fgets(line,sizeof(line),stdin))
    {
        sscanf(line,"%d-%d,%d-%d\n",&first_start,&first_end,&second_start,&second_end);
        if((first_start <= second_start && first_end >= second_end) ||
                (first_start >= second_start && first_end <= second_end))
            pair_counter++;
    }
    printf("first: %d\n",pair_counter);

    rewind(stdin);
    pair_counter = 0;

    while(fgets(line,sizeof(line),stdin))
    {
        sscanf(line,"%d-%d,%d-%d\n",&first_start,&first_end,&second_start,&second_end);
        if(first_start <= second_end && first_end >= second_start)
        {
            pair_counter++;
        }
    }
    printf("second: %d\n",pair_counter);

    return 0;
}

