#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

typedef uint32_t u32;

void maintain_top3(u32 top3[3], u32 newval)
{
    if(newval <= top3[0]) return;
    top3[0] = newval;
    for(int i=1; i<3; i++)
    {
        if(top3[i] >= top3[i-1]) return;
        u32 tmp = top3[i-1];
        top3[i-1] = top3[i];
        top3[i] = tmp;
    }
}

int main(void)
{
    char line[128];
    u32 top3[3] = {0};
    u32 this_elf_sum = 0;

    while(fgets(line,sizeof(line),stdin))
    {
        if(line[0] == '\n')
        {
            maintain_top3(top3,this_elf_sum);
            this_elf_sum = 0;
            continue;
        }
        this_elf_sum += strtoul(line,NULL,10);
    }

    printf("top elf: %d\n",top3[2]);
    printf("sum of top 3 elves: %d\n",top3[0]+top3[1]+top3[2]);
    return 0;
}

