use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

use anyhow::Context;

bindgen!("todo-list" in "./todo-list.wit");

#[rustler::nif(name = "load_component")]
fn load_component() -> Vec<u8> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();

    let component = Component::from_file(&engine, "./todo-list.wasm")
        .context("Component file not found")
        .unwrap();
    component.serialize().unwrap()
}

#[rustler::nif(name = "init")]
fn init_impl(serialized_component: Vec<u8>) -> Vec<String> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, ());

    let component = unsafe { Component::deserialize(&engine, serialized_component).unwrap() };

    let (instance, _) = TodoList::instantiate(&mut store, &component, &linker).unwrap();

    instance.call_init(&mut store).unwrap()
}

#[rustler::nif(name = "add_todo")]
fn add_todo_impl(
    serialized_component: Vec<u8>,
    todo: String,
    todo_list: Vec<String>,
) -> Vec<String> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, ());

    let component = unsafe { Component::deserialize(&engine, serialized_component).unwrap() };

    let (instance, _) = TodoList::instantiate(&mut store, &component, &linker).unwrap();

    instance
        .call_add_todo(&mut store, &todo, &todo_list[..])
        .unwrap()
}

rustler::init!("Elixir.TodoList", [init_impl, load_component, add_todo_impl]);
