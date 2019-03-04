import * as wasm from "textglue-wasm";

wasm.greet();
console.log(wasm.get_database());
wasm.set_snippet("abc","lorem");
console.log(wasm.get_database());
console.log(wasm.get_snippet("abc"));
console.log(wasm.get_metadata("abc"));
console.log(wasm.set_metadata("abc",123));
console.log(wasm.get_database_pretty_json());
console.log(wasm.set_database_json(wasm.get_database_pretty_json()));
