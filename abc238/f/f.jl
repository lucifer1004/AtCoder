const MOD = 998_244_353

function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

function solve()
    n, k = read_ints()
    p = read_ints()
    q = read_ints()
    citizens = collect(zip(p, q))
    sort!(citizens)

    dp = zeros(Int, k + 1, n + 1)
    dp[1, n+1] = 1

    for rk1 in 1:n # Rk1: 1 -> n
        ndp = zeros(Int, k + 1, n + 1)

        for rk2 in 1:n+1 # Rk2 Smallest of non-selected: 1 -> n + 1
            for num in 1:k+1 # Number of people selected: 0 -> k
                if dp[num, rk2] == 0
                    continue
                end

                # Select this citizen (if possible)
                if num <= k && citizens[rk1][2] < rk2
                    ndp[num+1, rk2] += dp[num, rk2]
                    ndp[num+1, rk2] %= MOD
                end

                # Do not select this citizen
                rk2′ = min(rk2, citizens[rk1][2])
                ndp[num, rk2′] += dp[num, rk2]
                ndp[num, rk2′] %= MOD
            end
        end

        dp = ndp
    end

    ans = sum(dp[k+1, :]) % MOD
    println(ans)
end

solve()
