defmodule WasmComponentDemoTest do
  use ExUnit.Case
  doctest WasmComponentDemo

  test "greets the world" do
    assert WasmComponentDemo.hello() == :world
  end
end
