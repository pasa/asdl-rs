# ASDL parser and CLI for code generation.

ASDL describes the abstract syntax of compiler intermediate representations and other tree-like data structures. Just as the lexical and syntactic structures of programming languages are described with regular expressions and context free grammars, ASDL provides a concise notation for describing the abstract syntax of programming languages. Tools can convert ASDL descriptions into the appropriate data-structure definitions and functions to convert the data-structures to or it easier to build compiler components that interoperate.

You can read about Asdl in this [paper](https://www.cs.princeton.edu/research/techreps/TR-554-97)

Functionality is provided in two crates:

* `asdl` - parser and api ready to use in code generation. Best suited for code generation with `rust` code. With `quota` for example.
* `asdl-tera` model designed for template processing engines like `tera` and a CLI for code generation with `tera` template engine.

