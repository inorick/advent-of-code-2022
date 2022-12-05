#include <string.h>
#include <stdio.h>
#include <assert.h>

#define STACKS 9

struct stack {
    int top;
    char values[64];
} stack[STACKS] = {0};

void stack_push(struct stack *s, char v)
{
    s->values[s->top++] = v;
}

char stack_pop(struct stack *s)
{
    return s->values[--s->top];
}

void stack_rev(struct stack *s)
{
    char tmp[16];
    memcpy(tmp,s->values,sizeof(tmp));
    for(int k=0, i=s->top-1; i>=0; i--,k++)
    {
        s->values[k] = tmp[i];
    }
}

void parse_crate_line(const char *line)
{
    for(int i=0; i<STACKS; i++)
    {
        char c = line[i*4 + 1];
        if(c != ' ')
        {
            stack_push(&stack[i],c);
        }
    }
}

int main(void)
{
    char line[64];
    for(int i=0; i<8; i++)
    {
        fgets(line,sizeof(line),stdin);
        parse_crate_line(line);
    }

    for(int i=0; i<STACKS; i++)
    {
        stack_rev(&stack[i]);
    }

#if 1
    for(int i=0; i<STACKS; i++)
    {
        printf("stack top: %d\n",stack[i].top);
        for(int k=0; k<stack[i].top; k++)
        {
            printf("%c ",stack[i].values[k]);
        }
        puts("");
    }
#endif

#if 1
    while(fgets(line,sizeof(line),stdin))
    {
        if(line[0] == '\n') continue;
        int n;
        int from;
        int to;
        sscanf(line,"move %d from %d to %d\n",&n,&from,&to);
        from--;
        to--;
        for(int i=0; i<n; i++)
        {
            char c = stack_pop(&stack[from]);
            stack_push(&stack[to],c);
        }

#if 0
        for(int i=0; i<STACKS; i++)
        {
            printf("\nstack top: %d\n",stack[i].top);
            for(int k=0; k<stack[i].top; k++)
            {
                printf("%c ",stack[i].values[k]);
            }
            puts("");
        }
#endif
    }

    for(int i=0; i<STACKS; i++)
    {
        printf("%c",stack[i].values[stack[i].top-1]);
    }
    puts("");
#endif

    return 0;
}

