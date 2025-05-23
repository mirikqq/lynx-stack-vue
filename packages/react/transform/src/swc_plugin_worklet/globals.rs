use once_cell::sync::Lazy;
use std::collections::HashSet;

pub static DEFAULT_GLOBALS: Lazy<HashSet<&str>> = Lazy::new(|| {
  let mut m = HashSet::new();

  m.insert("AggregateError");
  m.insert("arguments");
  m.insert("Array");
  m.insert("ArrayBuffer");
  m.insert("AsyncFunction");
  m.insert("AsyncGenerator");
  m.insert("AsyncGeneratorFunction");
  m.insert("AsyncIterator");
  m.insert("Atomics");
  m.insert("BigInt");
  m.insert("BigInt64Array");
  m.insert("BigUint64Array");
  m.insert("Boolean");
  m.insert("DataView");
  m.insert("Date");
  m.insert("decodeURI");
  m.insert("decodeURIComponent");
  m.insert("encodeURI");
  m.insert("encodeURIComponent");
  m.insert("Error");
  m.insert("escape");
  m.insert("eval");
  m.insert("EvalError");
  m.insert("FinalizationRegistry");
  m.insert("Float32Array");
  m.insert("Float64Array");
  m.insert("Function");
  m.insert("Generator");
  m.insert("GeneratorFunction");
  m.insert("globalThis");
  m.insert("Infinity");
  m.insert("Int16Array");
  m.insert("Int32Array");
  m.insert("Int8Array");
  m.insert("InternalError");
  m.insert("Intl");
  m.insert("isFinite");
  m.insert("isNaN");
  m.insert("Iterator");
  m.insert("JSON");
  m.insert("Map");
  m.insert("Math");
  m.insert("NaN");
  m.insert("Number");
  m.insert("Object");
  m.insert("parseFloat");
  m.insert("parseInt");
  m.insert("Promise");
  m.insert("Proxy");
  m.insert("RangeError");
  m.insert("ReferenceError");
  m.insert("Reflect");
  m.insert("RegExp");
  m.insert("Set");
  m.insert("SharedArrayBuffer");
  m.insert("String");
  m.insert("Symbol");
  m.insert("SyntaxError");
  m.insert("TypedArray");
  m.insert("TypeError");
  m.insert("Uint16Array");
  m.insert("Uint32Array");
  m.insert("Uint8Array");
  m.insert("Uint8ClampedArray");
  m.insert("undefined");
  m.insert("unescape");
  m.insert("URIError");
  m.insert("WeakMap");
  m.insert("WeakRef");
  m.insert("WeakSet");

  m.insert("null");
  m.insert("undefined");
  m.insert("this");
  m.insert("console");

  m
});
pub static LYNX_GLOBALS: Lazy<HashSet<&str>> = Lazy::new(|| {
  let mut m = HashSet::new();
  m.insert("lynx");
  m.insert("SystemInfo");
  m.insert("setTimeout");
  m.insert("setInterval");
  m.insert("clearTimeout");
  m.insert("clearInterval");
  m.insert("requestAnimationFrame");
  m.insert("cancelAnimationFrame");
  m.insert("NativeModules");
  m.insert("__LEPUS__");
  m.insert("__JS__");
  m.insert("__MAIN_THREAD__");
  m.insert("__BACKGROUND__");
  m.insert("__DEV__");
  m.insert("runOnMainThread");
  m.insert("runOnBackground");
  m
});
