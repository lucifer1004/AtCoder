function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

n, q = read_ints()
adj = [Int[] for _ in 1:n+1]

for _ in 1:q
    u, v = read_ints()
    push!(adj[u], v + 1)
    push!(adj[v+1], u)
end

stk = Int[1]
vis = fill(false, n + 1)
vis[1] = true

while !isempty(stk)
    u = pop!(stk)
    for v in adj[u]
        if !vis[v]
            vis[v] = true
            push!(stk, v)
        end
    end
end

println(vis[n+1] ? "Yes" : "No")
