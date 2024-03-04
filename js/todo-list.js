export function init() {
  return ['Hello', 'From Codebeam'];
}

export function addTodo(todo, todos) {
  return [todo, ...todos];
}