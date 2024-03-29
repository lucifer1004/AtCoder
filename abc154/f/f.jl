N = 2000005
MOD = 1000000007
fac = ones(Int, N)
for i in 2:N
    fac[i] = fac[i-1] * i % MOD
end
ifac = ones(Int, N)
ifac[N] = invmod(fac[N], MOD)
for i in N-1:-1:2
    ifac[i] = ifac[i+1] * (i + 1) % MOD
end

@inline function comb(n, k)
    return k == 0 || k == n ? 1 : k > n ? 0 : fac[n] * ifac[k] % MOD * ifac[n-k] % MOD
end

function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

# f(r, c) = ∑ᵢⱼC(i + j, i) = ∑ⱼC(r + j + 1, j + 1) = C(r + c + 2, r + 1) - C(r + c + 2, 0)
@inline function f(r, c)
    return comb(r + c + 2, r + 1) - 1
end

@inline function solve()
    r1, c1, r2, c2 = read_params(Int)
    ans = f(r2, c2) - f(r1 - 1, c2) - f(r2, c1 - 1) + f(r1 - 1, c1 - 1)
    while ans < 0
        ans += MOD
    end
    println(ans)
end

solve()
