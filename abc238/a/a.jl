function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

n = read_ints()[1]
println(n == 1 || n >= 5 ? "Yes" : "No")
