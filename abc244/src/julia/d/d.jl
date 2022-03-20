function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    d = Dict{String, Int}(
        "R G B" => 1,
        "R B G" => 2,
        "G R B" => 3,
        "G B R" => 4,
        "B R G" => 5,
        "B G R" => 6,
    )

    good = [
        1 0 0 1 1 0;
        0 1 1 0 0 1;
        0 1 1 0 0 1;
        1 0 0 1 1 0;
        1 0 0 1 1 0;
        0 1 1 0 0 1
    ]

    s = d[readline()]
    t = d[readline()]

    println(good[s, t] == 1 ? "Yes" : "No")
end

solve()
