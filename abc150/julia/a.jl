k, x = [parse(Int64, x) for x in split(readline(), " ")]
if k * 500 >= x
    println("Yes")
else
    println("No")
end