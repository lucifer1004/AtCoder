function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    read_params(Int)
    s = Set(read_params(Int))
    println(length(s))
end

solve()
