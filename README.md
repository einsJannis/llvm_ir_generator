# A LLVM Intermediate Representation Code Generator Library [WIP]

This library aims to implmenet generator code for the entire LLVM IR language in rust.

## Installation

```toml
[dependencies]
llvm_ir_generator = { git = "https://github.com/einsjannis/llvm_ir_generator" }
```

## Q&A

### Why should you use this library?

At the moment you shouldn't, this is still very much WIP.

Incase I get this done one day this should be usefull though.
This library will allow you to easily write LLVM IR Code in a safe maner,
thanks to this library beeing type safe.
I will also try to document this project as well as possible,
to allow you never having to learn actuall LLVM IR
and just beeing able to start writing it with this library.

### What is LLVM Intermediate Representation and what is it good for?

LLVM is the compiler backend of the C compiler and many others.
LLVM IR is the language into which C gets compiled first before it's given to LLVM,
which compiles it down to asm for your prefered target.

LLVM is very usfull in the following cases:
 - You want to build your own compiler,
 but don't want to learn asm.
 - You want to make your compiler able to compile to a lot of targets,
 but you havn't got the time to implement all of them.
 - You don't want to write your own code optimizations.

because it handles all of this for you.

