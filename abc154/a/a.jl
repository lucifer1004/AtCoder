function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    s, t = split(readline())
    a, b = read_params(Int)
    u = readline()
    if u == s
        println(a - 1, " ", b)
    else
        println(a, " ", b - 1)
    end
end

solve()
