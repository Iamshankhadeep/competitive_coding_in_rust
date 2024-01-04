#include <bits/stdc++.h>

using namespace std;

template <typename A, typename B>
ostream &operator<<(ostream &os, const pair<A, B> &p) { return os << '(' << p.first << ", " << p.second << ')'; }
template <typename T_container, typename T = typename enable_if<!is_same<T_container, string>::value, typename T_container::value_type>::type>
ostream &operator<<(ostream &os, const T_container &v)
{
    os << '{';
    string sep;
    for (const T &x : v)
        os << sep << x, sep = ", ";
    return os << '}';
}
void dbg_out() { cerr << endl; }
template <typename Head, typename... Tail>
void dbg_out(Head H, Tail... T)
{
    cerr << ' ' << H;
    dbg_out(T...);
}
#ifdef LOCAL
#define dbg(...) cerr << "(" << #__VA_ARGS__ << "):", dbg_out(__VA_ARGS__)
#else
#define dbg(...)
#endif

#define ar array
#define ll long long
#define ld long double
#define sza(x) ((int)x.size())
#define all(a) (a).begin(), (a).end()

const int MAX_N = 1e5 + 5;
const ll MOD = 1e9 + 7;
const ll INF = 1e9;
const ld EPS = 1e-9;

long long gcd(long long int a, long long int b)
{
    if (b == 0)
        return a;
    return gcd(b, a % b);
}

// Function to return LCM of two numbers
long long lcm(ll a, ll b)
{
    return (a / gcd(a, b)) * b;
}

void solve()
{
    int n;
    cin >> n;
    vector<char> a(n);
    for (int i = 0; i < n; i++)
    {
        cin >> a[i];
    }
    if (n < 3)
    {
        for (int i = 0; i < n; i++)
        {
            cout << a[i];
        }
        cout << endl;
        return;
    }
    for (int i = 0; i < n; i += 2)
    {
        cout << a[i];
        if (i + 1 < n)
        {
            cout << a[i + 1];
        }
        if (i + 3 < n)
        {
            if (a[i + 3] == 'a' || a[i + 3] == 'e')
            {
                cout << '.';
            }
            else
            {
                cout << a[i + 2] << '.';
                i++;
            }
        }
    }
    cout << endl;
    return;
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    int tc;
    cin >> tc;
    for (int t = 1; t <= tc; t++)
    {
        // cout << "Case #" << t << ": ";
        solve();
    }
}