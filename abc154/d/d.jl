function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    n, k = read_params(Int)
    p = map(x -> (1.0 + x) / 2, read_params(Int))
    ans = 0.0
    s = 0.0
    for i in 1:n
        s += p[i]
        if i >= k + 1
            s -= p[i-k]
        end
        if i >= k
            ans = max(ans, s)
        end
    end
    println(ans)
end

solve()
