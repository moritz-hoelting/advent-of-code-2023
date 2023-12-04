defmodule Day04 do
  use Application

  def start(_type, _args) do
    input = File.read!("./input.txt")
    IO.puts "Part 1: " <> to_string(part1(input))
    IO.puts "Part 2: " <> to_string(part2(input))

    Supervisor.start_link [], strategy: :one_for_one
  end

  def part1(input) do
    input |> String.split("\n") |> Enum.map(&line_part1/1) |> Enum.sum
  end

  def part2(input) do
    card_values = input |> String.split("\n") |> Enum.map(&line_part2/1)

    card_amounts_initial = Enum.map(card_values, fn _ -> 1 end) |> Enum.with_index

    card_amounts_processed = Enum.reduce(Enum.map(card_amounts_initial, fn {_, i} -> i end), card_amounts_initial, fn i, acc ->
      {cur, _} = Enum.fetch!(acc, i)
      value = Enum.fetch!(card_values, i)

      Enum.map(acc, fn {cur2, i2} ->
        if value > 0 and i2 in (i+1)..(i+value) do
          {cur2 + cur, i2}
        else
          {cur2, i2}
        end
      end)
    end)

    card_amounts_processed |> Enum.map(fn {x, _} -> x end) |> Enum.sum
  end

  def line_part1(input) do
    [_, numbers] = String.split(input, ": ")
    [winning, own] = String.split(numbers, " | ") |> Enum.map(&String.split/1)
    matching = Enum.map(own, &(&1 in winning)) |> Enum.count(&(&1))

    if matching > 0 do
      :math.pow(2, matching - 1) |> round
    else
      0
    end
  end

  def line_part2(input) do
    [_, numbers] = String.split(input, ": ")
    [winning, own] = String.split(numbers, " | ") |> Enum.map(&String.split/1)

    Enum.map(own, &(&1 in winning)) |> Enum.count(&(&1))
  end
end
