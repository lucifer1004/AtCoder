defmodule Main do
  def main do
    IO.read(:line)
    IO.puts(length(String.split(IO.read(:line), "ABC")) - 1)
  end
end
