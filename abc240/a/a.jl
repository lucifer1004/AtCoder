function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    a, b = read_params(Int)
    if abs(a - b) == 1 || abs(a - b) == 9
        println("Yes")
    else
        println("No")
    end
end

solve()
