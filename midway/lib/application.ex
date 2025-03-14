defmodule Midway.Application do
  use Application

  def start(_type, _args) do
    children = [
      Midway.Counter,
      {Plug.Cowboy, scheme: :http, plug: Midway.Web, options: [port: 8080]}
    ]

    opts = [strategy: :one_for_one, name: Midway.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
