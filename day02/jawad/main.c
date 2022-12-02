#include <stdint.h>
#include <stdio.h>

typedef uint32_t u32;

static const u32 outcomes[3][3] = {
    4,8,3,
    1,5,9,
    7,2,6,
};

static const char secondstrat[3][3] = {
    'Z','X','Y',
    'X','Y','Z',
    'Y','Z','X',
};

int main(void)
{
    char line[8];
    u32 score = 0;

    while(fgets(line,sizeof(line),stdin))
    {
        score += outcomes[line[0] - 'A'][line[2] - 'X'];
    }
    printf("1st strat: %d\n",score);

    rewind(stdin);
    score = 0;

    while(fgets(line,sizeof(line),stdin))
    {
        score += outcomes[line[0] - 'A'][secondstrat[line[0] - 'A'][line[2] - 'X'] - 'X'];
    }
    printf("2nd strat: %d\n",score);

    return 0;
}

