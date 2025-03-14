defmodule MidwayTest do
  use ExUnit.Case
  doctest Midway

  test "greets the world" do
    assert Midway.hello() == :world
  end
end
