# wasm_component_demo
WebAssembly Component Example for Codebeam 2024

## Running it

```
mix deps.get
```

```
iex> comp = TodoList.load_component()
iex(2)> list = TodoList.init(comp)
["Hello", "WASM Components"]
iex(3)> list = TodoList.add_todo(comp, "hi", list)
["hi", "Hello", "WASM Components"]
```
