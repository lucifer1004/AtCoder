const MOD = 998_244_353

function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

function mod_exp(x::Int, y::Int)::Int
    ans = 1

    while y > 0
        if y % 2 == 1
            ans = (ans * x) % MOD
        end
        x = (x * x) % MOD
        y >>= 1
    end

    ans
end

@inline function mod_inv(x::Int)::Int
    mod_exp(x, MOD - 2)
end

function solve()
    n = read_ints()[1]
    fac = accumulate((x, y) -> x * y % MOD, 1:n)
    ifac = fill(mod_inv(fac[n]), n)
    for i in n-1:-1:1
        ifac[i] = ifac[i+1] * (i + 1) % MOD
    end

    @inline function Cₙᵏ(n::Int, k::Int)::Int
        n == k || k == 0 ? 1 : fac[n] * ifac[k] % MOD * ifac[n-k] % MOD
    end

    s = readline()
    nums = zeros(Int, n, n)
    cost = zeros(Int, n, n)
    for i in 1:n
        nums[i, i%n+1] = 1
    end

    for len in 3:n+1
        for l in 1:n
            r = l + len - 1
            if r > n
                r -= n
            end

            # Choices of the last step
            # Remove the middle person from left or from right
            c₁ = (s[l] == 'R') + (s[r] == 'L')

            for m in l+1:l+len-2
                mid = m <= n ? m : m - n
                c₂ = (m - l) * (s[l] == 'R') + (l + len - 1 - m) * (s[r] == 'L')
                comb = Cₙᵏ(len - 3, m - l - 1)
                nums[l, r] += nums[l, mid] * nums[mid, r] % MOD * c₁ % MOD * comb % MOD
                nums[l, r] %= MOD
                cost[l, r] += (nums[l, mid] * nums[mid, r] % MOD * c₂ % MOD + nums[l, mid] * cost[mid, r] % MOD * c₁ % MOD + cost[l, mid] * nums[mid, r] % MOD * c₁ % MOD) * comb % MOD
                cost[l, r] %= MOD
            end
        end
    end

    println(sum(cost[i, i] for i in 1:n) % MOD * ifac[n] % MOD)
end

solve()
