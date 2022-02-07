function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

n = read_ints()[1]
angles = read_ints()

cuts = Set{Int}()
push!(cuts, 0)
now = Ref(0)
for angle in angles
    now[] = (now[] + angle) % 360
    push!(cuts, now[])
end

cuts = [sort(collect(cuts))..., 360]
println(maximum(y - x for (x, y) in zip(cuts[1:end-1], cuts[2:end])))
