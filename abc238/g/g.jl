using Random

function read_ints()::Vector{Int}
    line = readline()
    return map(x -> parse(Int, x), split(line))
end

function gen_primes(upper::Int)::Vector{Int}
    primes = Int[]
    mark = fill(true, upper)

    for i in 2:upper
        if mark[i]
            push!(primes, i)
        end

        for prime in primes
            if i * prime > upper
                break
            end

            mark[i*prime] = false
            if i % prime == 0
                break
            end
        end
    end

    primes
end

function decompose(num::Int, primes::Vector{Int})::Vector{Pair{Int,Int}}
    decomposition = Pair{Int,Int}[]
    for prime in primes
        if num % prime == 0
            cnt = 0
            while num % prime == 0
                num ÷= prime
                cnt += 1
            end
            if cnt % 3 != 0
                push!(decomposition, prime => cnt % 3)
            end
        end

        if num < prime * prime
            break
        end
    end

    if num != 1
        push!(decomposition, num => 1)
    end

    decomposition
end

function solve()
    Random.seed!(42)

    n, q = read_ints()
    a = read_ints()
    hi = maximum(a)
    primes = gen_primes(hi)
    Nₚ = length(primes)
    flags = Dict{Int,NTuple{3,UInt}}()
    sizehint!(flags, Nₚ)
    for prime in primes
        X₀ = rand(UInt)
        X₁ = rand(UInt)
        X₂ = X₀ ⊻ X₁
        push!(flags, prime => (X₀, X₁, X₂))
    end

    hashes = zeros(UInt, n + 1)
    prime_counter = zeros(Int, hi)
    decompositions = map(x -> decompose(x, primes), a)
    for i in 1:n
        for (prime, cnt) in decompositions[i]
            for _ in 1:cnt
                prime_counter[prime] += 1
                hashes[i+1] ⊻= flags[prime][prime_counter[prime]%3+1]
            end
        end
    end

    prefix = accumulate(⊻, hashes)
    ranges = [read_ints() for _ in 1:q]
    ans = Bool[]
    for (l, r) in ranges
        push!(ans, prefix[r+1] == prefix[l])
    end

    println(join(map(x -> x ? "Yes" : "No", ans), "\n"))
end

solve()
