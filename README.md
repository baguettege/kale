# Kale

A dynamically typed scripting language written in Rust.

## Features
- Dynamic typing, everything is an object
- First-class functions and closures
- Modules, structs
- Lists and strings
- Extensible via Rust native bindings
- Spanned error reporting
- Comments

## Usage
Compile to `.kast`:
```bash
kale compile main.kale
```

Run:
```bash
kale run main.kast
```

Exec (compile and run):
```bash
kale exec main.kale
```

Show (source code):
```bash
kale show main.kast
```

## Syntax
```kale
# variable declaration
let n = nil;
let num = 3.14;
let c = 'a';
let str = "hello world";
let list = [1, '2', "3", num, c];

# assigning
num = nil;

# control flow
while cond {
    # logic...
}

if cond and other {
    # logic...
} else {
    # logic...
}

# free functions
fn add(a, b) {
    return a + b;
}

assert(add(1, 2) == 3);

# closures
let add = |a, b| {
    return a + b;
};

assert(add("ab", "c") == "abc");

# modules
module counter {
    fn new() {
        let count = 0;
        return || {
            count = count + 1;
            return count;
        };
    }
}

let c = counter.new();
assert(c() == 1);

# structs
struct Iter(list, _i) {
    fn new(list) {
        return Iter(list, 0);
    }
    
    fn next(self) {
        let i = self._i;
        self._i = i + 1;
        return self.list.get(i);
    }
    
    fn has_next(self) {
        return self._i < self.list.len();
    }
}

let list = [1, 2];
let iter = Iter.new(list);
assert(iter.next() == 1);
assert(iter.next() == 2);
assert(iter.next() == nil);

# errors (terminates execution with a message)
raise "some error";
raise 1;
```

## Architecture
- `kale-syntax` - AST and token definitions
- `kale-lexer` - tokenizer
- `kale-parser` - parser
- `kale-codec` - .kast file codec
- `kale-runtime` - object model and type system
- `kale-interpreter` - tree walking interpreter
- `kale-report` - error rendering
- `kale-api` - public API
- `kale-stdlib` - standard library
- `kale` - main binary

## Operators
- `*`, `/`, `%`, `+`, `-`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `is`, `and`, `or`, `not`
- `is` - reference equality
- `==` - value equality

## Type methods
- `num` - `to_str`, `min`, `max`, `abs`, `floor`, `ceil`, `round`
- `char` - `to_str`, `is_digit`, `is_whitespace`, `is_alphabetic`, `is_alphanumeric`
- `str` - `clone`, `len`, `char_at`, `chars`, `slice`, `to_lower`, `to_upper`, `trim`,
          `contains`, `split`, `find`, `starts_with`, `ends_with`, `replace`, `to_num`
- `list` - `len`, `is_empty`, `get`, `set`, `push`, `pop`, `clone`, `to_str`

## Stdlib
- `type`, `print`, `println`, `readln`, `assert`, `or_else`
