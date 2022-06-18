function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function comb(n, k)
    if k > n
        return 0
    elseif k == 0
        return 1
    elseif k == 1
        return n
    elseif k == 2
        return n * (n - 1) ÷ 2
    elseif k == 3
        return n * (n - 1) * (n - 2) ÷ 6
    else
        return 0 # Impossible in this problem
    end
end

function solve()
    n = readline()
    k = read_params(Int)[1]

    m = length(n)
    ans = 0
    eqnz = 0
    for i in 1:m
        p = n[i] - '0'

        if p > 0
            ans += comb(m - i, k - eqnz) * 9^(k - eqnz) # Set current digit to 0
            eqnz += 1
            if eqnz > k
                break
            end
        end

        for _ in 1:p-1
            ans += comb(m - i, k - eqnz) * 9^(k - eqnz) # Set current digit to 1..p-1
        end
    end

    if eqnz == k
        ans += 1
    end
    println(ans)
end

solve()
