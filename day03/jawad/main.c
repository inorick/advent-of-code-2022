#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

typedef uint64_t u64;

u64 item_bitmask(const char *line)
{
    u64 mask = 0;
    for(size_t i=0; line[i] && line[i] != '\n'; i++)
    {
        if (line[i] <= 'Z') mask |= (u64)0x4000000 << (line[i] - 'A');
        else mask |= (u64)0x01 << (line[i] - 'a');
    }
    return mask;
}

int main(void)
{
    size_t sum = 0;
    char line[64];
    u64 common = 0xffffffffffffffff;
    size_t elf_counter = 0;
    while(fgets(line,sizeof(line),stdin))
    {
        common &= item_bitmask(line);
        elf_counter++;
        if(elf_counter == 3)
        {
            elf_counter = 0;
            for(u64 mask=0x01, i=1; i<64; i++, mask<<=1)
            {
                if(common & mask) sum += i;
            }
            common = 0xffffffffffffffff;
        }
    }
    printf("answer: %zu\n",sum);
    return 0;
}

