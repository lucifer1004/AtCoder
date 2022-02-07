function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

t = read_ints()[1]
for _ in 1:t
    a, s = read_ints()
    s -= 2 * a
    if s < 0
        println("No")
        continue
    end

    valid = true
    for k in 60:-1:0
        msk = 1 << k
        if (s & msk) != 0 && (a & msk) != 0
            println("No")
            valid = false
            break
        end
    end

    if valid
        println("Yes")
    end
end