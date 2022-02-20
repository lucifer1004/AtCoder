function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    n = read_params(Int)[1]
    a = read_params(Int)
    stk = Pair{Int,Int}[]
    tot = 0
    for ai in a
        if isempty(stk) || ai != stk[end].first
            push!(stk, ai => 1)
            tot += 1
        else
            if stk[end].second + 1 == stk[end].first
                tot -= stk[end].second
                pop!(stk)
            else
                stk[end] = stk[end].first => stk[end].second + 1
                tot += 1
            end
        end
        println(tot)
    end
end

solve()
