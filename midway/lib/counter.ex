defmodule Midway.Counter do
  use Agent

  def start_link(_opts \\ []) do
    Agent.start_link(fn -> 0 end, name: __MODULE__)
  end

  def get do
    Agent.get(__MODULE__, & &1)
  end

  def inc do
    Agent.get_and_update(__MODULE__, fn value ->
      new_value = value + 1
      {new_value, new_value}
    end)
  end

  def dec do
    Agent.get_and_update(__MODULE__, fn value ->
      new_value = value - 1
      {new_value, new_value}
    end)
  end
end
