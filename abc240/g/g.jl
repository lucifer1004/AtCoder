function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

const MOD = 998_244_353

function solve(n::Int, x::Int, y::Int, z::Int)
    if x + y + z > n || (x + y + z - n) % 2 != 0
        println(0)
        return
    end

    fac = ones(Int, n)
    for i in 2:n
        fac[i] = fac[i-1] * i % MOD
    end

    ifac = ones(Int, n)
    ifac[end] = invmod(fac[end], MOD)

    for i in n-1:-1:1
        ifac[i] = ifac[i+1] * (i + 1) % MOD
    end

    @inline function comb(n::Int, k::Int)
        if k == 0 || k == n
            return 1
        end

        return fac[n] * ifac[k] % MOD * ifac[n-k] % MOD
    end

    @inline function f₁(n::Int, x::Int)
        return comb(n, (n - abs(x)) ÷ 2)
    end

    @inline function f₂(n::Int, x::Int, y::Int)
        return f₁(n, x + y) * f₁(n, x - y) % MOD
    end

    ans = 0
    for ze in 0:(n-(x+y+z))÷2
        ans += comb(n, z + 2ze) * f₁(z + 2ze, z) % MOD * f₂(n - z - 2ze, x, y) % MOD
    end

    println(ans % MOD)
end

n, x, y, z = read_params(Int)
solve(n, abs(x), abs(y), abs(z))
