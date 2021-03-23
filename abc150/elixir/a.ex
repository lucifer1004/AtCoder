defmodule Main do
  def main do
    [k, x] =
      IO.read(:line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(fn n -> String.to_integer(n) end)

    if k * 500 >= x do
      IO.puts("Yes")
    else
      IO.puts("No")
    end
  end
end
