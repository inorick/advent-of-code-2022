#include <iostream>
#include <vector>

using isz = __int128;

using namespace std;

class Monkey {
public:
    Monkey(initializer_list<isz> i, function<void(isz&)> fnnew, function<int(isz)> fntest) : items(i), opnew(fnnew), test(fntest), count(0) {}
    vector<isz> items;
    function<void(isz&)> opnew;
    function<int(isz)> test;
    int count;
};

void playround(vector<Monkey>& monkeys)
{
    for(auto& m : monkeys)
    {
        if(m.items.size() == 0) continue;

        for(auto& i : m.items) m.opnew(i);

        for(auto i : m.items) monkeys[m.test(i)].items.push_back(i);

        m.count += m.items.size();
        m.items.clear();
    }
}

int main(void)
{
    vector<Monkey> monkeys = {
        /*
        {{71, 56, 50, 73}, [](int& x){ x *= 11; }, [](int x){ return x % 13 == 0 ? 1 : 7; }},
        {{70, 89, 82}, [](int& x){ x += 1; }, [](int x){ return x % 7 == 0 ? 3 : 6; }},
        {{52, 95}, [](int& x){ x *= x; }, [](int x){ return x % 3 == 0 ? 5 : 4; }},
        {{94, 64, 69, 87, 70}, [](int& x){ x += 2; }, [](int x){ return x % 19 == 0 ? 2 : 6; }},
        {{98, 72, 98, 53, 97, 51}, [](int& x){ x += 6; }, [](int x){ return x % 5 == 0 ? 0 : 5; }},
        {{79}, [](int& x){ x += 7; }, [](int x){ return x % 2 == 0 ? 7 : 0; }},
        {{77, 55, 63, 93, 66, 90, 88, 71}, [](int& x){ x *= 7; }, [](int x){ return x % 11 == 0 ? 2 : 4; }},
        {{54, 97, 87, 70, 59, 82, 59}, [](int& x){ x += 8; }, [](int x){ return x % 17 == 0 ? 1 : 3; }}
        */
        {{79, 98}, [](isz& x){ x *= 19; }, [](int x){ return x % 23 == 0 ? 2 : 3; }},
        {{54, 65, 75, 74}, [](isz& x){ x += 6; }, [](int x){ return x % 19 == 0 ? 2 : 0; }},
        {{79, 60, 97}, [](isz& x){ x *= x; }, [](int x){ return x % 13 == 0 ? 1 : 3; }},
        {{74}, [](isz& x){ x += 3; }, [](int x){ return x % 17 == 0 ? 0 : 1; }}
    };

    for(int i=0; i<20; i++) playround(monkeys);
    for(auto& m : monkeys) cout << m.count << endl;
    return 0;
}

