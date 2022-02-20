function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    n, x = read_params(Int)
    steps = [read_params(Int) for _ in 1:n]
    can = fill(false, x + 1)
    can[1] = true
    for (a, b) in steps
        nxt = fill(false, x + 1)
        for last in eachindex(can)
            if can[last]
                if last + a <= x + 1
                    nxt[last+a] = true
                end
                if last + b <= x + 1
                    nxt[last+b] = true
                end
            end
        end
        can = nxt
    end

    if can[x+1]
        println("Yes")
    else
        println("No")
    end
end

solve()
