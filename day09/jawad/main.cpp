#include <iostream>
#include <unordered_map>

using namespace std;

class pt {
public:
    int x;
    int y;

    pt(int ix=0, int iy=0): x(ix), y(iy) {}

    pt operator+(const pt& p)
    {
        return pt(x+p.x,y+p.y);
    }

    pt operator-(const pt& p)
    {
        return pt(x-p.x,y-p.y);
    }

    operator string() { return to_string(x) + "," + to_string(y); }
};

int main(void)
{
    unordered_map<string,pt> nextmove = {
        {"0,0", pt(0,0)},
        {"0,1", pt(0,0)},
        {"1,0", pt(0,0)},
        {"0,-1", pt(0,0)},
        {"-1,0", pt(0,0)},
        {"1,-1", pt(0,0)},
        {"-1,-1", pt(0,0)},
        {"-1,1", pt(0,0)},
        {"1,1", pt(0,0)},
        {"2,0", pt(1,0)},
        {"2,1", pt(1,1)},
        {"1,2", pt(1,1)},
        {"2,2", pt(1,1)},
        {"0,2", pt(0,1)},
        {"-1,2", pt(-1,1)},
        {"-2,1", pt(-1,1)},
        {"-2,2", pt(-1,1)},
        {"-2,0", pt(-1,0)},
        {"-2,-1", pt(-1,-1)},
        {"-1,-2", pt(-1,-1)},
        {"-2,-2", pt(-1,-1)},
        {"0,-2", pt(0,-1)},
        {"1,-2", pt(1,-1)},
        {"2,-1", pt(1,-1)},
        {"2,-2", pt(1,-1)},
    };

    unordered_map<string,bool> places;

    const int N = 10;   //set to 2 for part 1

    pt rope[N] = {pt(0,0)};

    char line[16];
    while(fgets(line,sizeof(line),stdin))
    {
        pt move;
        switch(line[0])
        {
            case 'R': move = pt(1,0); break;
            case 'L': move = pt(-1,0); break;
            case 'U': move = pt(0,-1); break;
            case 'D': move = pt(0,1); break;
        }

        int n = strtol(line+2,NULL,10);
        for(int i=0; i<n; i++)
        {
            rope[0] = rope[0] + move;
            for(int r=1; r<N; r++)
            {
                rope[r] = rope[r] + nextmove[(string)(rope[r-1] - rope[r])];
            }
            places[(string)rope[N-1]] = true;
        }
    }

    cout << places.size() << endl;

    return 0;
}

