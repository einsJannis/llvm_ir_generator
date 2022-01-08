# A LLVM IR Code Generator Library [WIP]

This library aims to implement generator code for the entire LLVM intermediate representation language in rust.

## Installation

Add the following to your `Cargo.toml` of your rust project
```toml
[dependencies]
llvm_ir_generator = { git = "https://github.com/einsjannis/llvm_ir_generator" }
```

## Q&A

### Why should you use this library?

At the moment you shouldn't, this is still very much WIP.

In case I get this done one day this should be useful though.
This library will allow you to easily write LLVM IR Code in a safe manner,
thanks to this library being type safe.
I will also try to document this project as well as possible,
to allow you never having to learn actual LLVM IR
and just being able to start writing it with this library.

### What is LLVM Intermediate Representation and what is it good for?

LLVM is the compiler backend of the C compiler and many others.
LLVM IR is the language into which C gets compiled first before it's given to LLVM,
which compiles it down to asm for your preferred target.

LLVM is very useful in the following cases:
 - You want to build your own compiler,
 but don't want to learn asm.
 - You want to make your compiler able to compile to a lot of targets,
 but you haven't got the time to implement all of them.
 - You don't want to write your own code optimizations.

because it handles all of this for you.
