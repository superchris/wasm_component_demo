defmodule TodoList do
  use Rustler, otp_app: :wasm_component_demo, crate: "todo_list"

  # When your NIF is loaded, it will override this function.
  def init(serialized_component), do: error()
  def add_todo(serialized_component, todo, todo_list), do: error()

  def load_component(), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
