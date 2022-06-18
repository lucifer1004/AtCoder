function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    n = read_params(Int)[1]
    a = read_params(Int)
    if length(Set(a)) == n
        println("YES")
    else
        println("NO")
    end
end

solve()
