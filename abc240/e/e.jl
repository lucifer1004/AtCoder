function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function dfs(u::Int, p::Int, curr::Base.RefValue{Int}, adj::Vector{Vector{Int}}, lb::Vector{Int}, ub::Vector{Int})
    lb[u] = curr[]

    child_count = 0
    for v in adj[u]
        if v != p
            child_count += 1
            dfs(v, u, curr, adj, lb, ub)
            curr[] = ub[v] + 1
        end
    end

    if child_count >= 1
        curr[] -= 1
    end

    ub[u] = curr[]
end

function solve()
    n = read_params(Int)[1]
    edges = [read_params(Int) for _ in 1:n-1]
    lb = zeros(Int, n)
    ub = zeros(Int, n)
    adj = [Int[] for _ in 1:n]
    for (u, v) in edges
        push!(adj[u], v)
        push!(adj[v], u)
    end

    dfs(1, 0, Ref(1), adj, lb, ub)
    for i in 1:n
        println(lb[i], " ", ub[i])
    end
end

solve()
