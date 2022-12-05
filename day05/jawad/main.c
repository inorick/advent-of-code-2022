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
    char tmp[sizeof(stack[0].values)];
    memcpy(tmp,s->values,sizeof(tmp));
    for(int k=0, i=s->top-1; i>=0; i--,k++)
    {
        s->values[k] = tmp[i];
    }
}

void stack_move(struct stack *from, struct stack *to, int n)
{
    struct stack tmp = {0};
    for(int i=0; i<n; i++)
    {
        stack_push(&tmp,stack_pop(from));
    }
    for(int i=0; i<n; i++)
    {
        stack_push(to,stack_pop(&tmp));
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

    while(fgets(line,sizeof(line),stdin))
    {
        if(line[0] == '\n') continue;
        int n;
        int from;
        int to;
        sscanf(line,"move %d from %d to %d\n",&n,&from,&to);
        from--;
        to--;
#if 0
        for(int i=0; i<n; i++)
        {
            char c = stack_pop(&stack[from]);
            stack_push(&stack[to],c);
        }
#else
        stack_move(&stack[from],&stack[to],n);
#endif
    }

    for(int i=0; i<STACKS; i++)
    {
        printf("%c",stack[i].values[stack[i].top-1]);
    }
    puts("");

    return 0;
}

