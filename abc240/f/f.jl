function read_params(T::Type)
    line = readline()
    return map(x -> parse(T, x), split(line))
end

function solve()
    n, m = read_params(Int)
    parts = [read_params(Int) for i in 1:n]

    ans = parts[1][1]
    b = 0
    pos = 0
    now = 0
    for (x, y) in parts
        lv = now + (b + x)
        rv = now + b * y + y * (y + 1) ÷ 2 * x
        ans = max(ans, lv)
        ans = max(ans, rv)

        lb = b + x
        rb = b + y * x
        if lb > 0 && rb < 0
            mid = b ÷ abs(x)
            mv = now + b * mid + mid * (mid + 1) ÷ 2 * x
            ans = max(ans, mv)
        end

        b += y * x
        pos += y
        now = rv
    end

    println(ans)
end

t = read_params(Int)[1]
for _ in 1:t
    solve()
end
