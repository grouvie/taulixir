defmodule Midway.Application do
  use Application

  def start(_type, _args) do
    children = [
      Midway.Counter,
    ]

    opts = [strategy: :one_for_one, name: Midway.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
