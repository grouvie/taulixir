defmodule Midway.Web do
  use Plug.Router

  plug(:match)
  plug(:dispatch)

  get "/" do
    counter = Midway.Counter.get()
    send_resp(conn, 200, "#{counter}")
  end

  get "/increase" do
    counter = Midway.Counter.inc()
    send_resp(conn, 200, "#{counter}")
  end

  get "/decrease" do
    counter = Midway.Counter.dec()
    send_resp(conn, 200, "#{counter}")
  end

  match _ do
    send_resp(conn, 404, "Not Found")
  end
end
