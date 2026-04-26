# Kale

A dynamically typed scripting language written in Rust.

## Features
- Dynamic typing, everything is an object
- First-class functions and closures
- Modules
- Lists and strings
- Builtin methods and stdlib functions

## Usage
Compile `.kale` to `.kast`:
```bash
kalec main
```

Run `.kast`:
```bash
kale main
```

## Syntax
```kale
n = nil;
x = 3.14;
b = true;
s = "hello world!";
l = ["str", 1.718];
f = fn() {};

fn add(a, b) {
    return a + b;
}

while not (b is false) {
    b = false;
}

if b and true {
    n = 1;
} else {
    n = 2;
}

module counter {
    fn new() {
        count = 0;
        return fn() {
            count = count + 1;
            return count;
        }
    }
}

c = counter.new();
io.println(c(), c(), c());
```

## Architecture
- `kale-syntax` - AST and token definitions
- `kale-lexer` - tokenizer
- `kale-parser` - parser
- `kale-codec` - binary AST codec
- `kale-runtime` - object model and type system
- `kale-interpreter` - tree walking interpreter
- `kale-api` - public API
- `kale-stdlib` - standard library

## Stdlib
- `core` - `type_of`, `num`, `bool`, `str`
- `io` - `print`, `println`
- `math` - `floor`, `ceil`, `sqrt`, `abs`

`str` methods: `len`, `to_upper`, `to_lower`, `at`

`list` methods: `len`, `push`, `pop`, `get`

## Why
I built Kale to understand how programming languages work under the hood – the object model, closures, environments, compilation, and how it all fits together.
The design follows a simple Python-like principle: everything is an object. No special cases for anything, just a uniform model where all values live in the same environment.
