function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

n = read_ints()[1]
bounds = Int[1, 10]
ans = Ref{Int128}(0)
while n >= bounds[2]
    r = bounds[2] - bounds[1]
    ans[] += Int128(r) * (r + 1) // 2
    bounds .*= 10
end

r = n - bounds[1] + 1
ans[] += Int128(r) * (r + 1) // 2

println(ans[] % 998244353)
