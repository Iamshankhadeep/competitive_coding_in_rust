#include "bits/stdc++.h"
using namespace std;
#define sim template <class c
#define ris return *this
#define dor > debug &operator<<
#define eni(x)                                                                    \
    sim > typename enable_if<sizeof dud<c>(0) x 1, debug &>::type operator<<(c i) \
    {
sim > struct rge
{
    c b, e;
};
sim > rge<c> range(c i, c j) { return rge<c>{i, j}; }
sim > auto dud(c *x) -> decltype(cerr << *x, 0);
sim > char dud(...);
struct debug
{
#ifdef LOCAL
    ~debug() { cerr << endl; }
    eni(!=) cerr << boolalpha << i;
    ris;
} eni(==) ris << range(begin(i), end(i));
}
sim, class b dor(pair<b, c> d)
{
    ris << "(" << d.first << ", " << d.second << ")";
}
sim dor(rge<c> d)
{
    *this << "[";
    for (auto it = d.b; it != d.e; ++it)
        *this << ", " + 2 * (it == d.b) << *it;
    ris << "]";
}
#else
    sim dor(const c &) { ris; }
#endif
}
;
#define imie(...) " [" << #__VA_ARGS__ ": " << (__VA_ARGS__) << "] "

using ll = long long;
vector<int> v;
int main()
{
    int n, a, b, c, d;
    cin >> n >> a >> b >> c >> d;
    if ((a == 0 && c == 0) || (b == 0 && d == 0) || (a == n && c == n) || (b == n && d == n))
    {
        cout << abs(a - c) + abs(b - d) << endl;
        return 0;
    }
    else
    {
        int x = min(abs(a - 0) + abs(b - 0) + abs(c - 0) + abs(d - 0), abs(a - n) + abs(b - n) + abs(c - n) + abs(d - n));
        int y = min(abs(a - 0) + abs(b - n) + abs(c - 0) + abs(d - n), abs(a - n) + abs(b - 0) + abs(c - n) + abs(d - 0));
        cout << min(x, y) << endl;
    }
}
