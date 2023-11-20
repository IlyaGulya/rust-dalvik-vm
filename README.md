# Rust Dalvik VM

## Introduction

This is a project I'm writing purely for educational purposes.

## Plan

- [x] Parse DEX file (I'm aware of `dexparser` and `dex` crates, but I want to do it myself to better understand DEX
  file structure)
- [ ] Be able to interpret simple DEX file (`hello.dex` in `tests/vm/hello.dex`, compiled
  from `tests/interpreter/hello.smali`)
- [ ] Add more tests for DEX file parser.
- [ ] Memory allocations?
- [ ] Garbage collection?
- [ ] JIT?
- [ ] AOT?
- [ ] JNI?
- [ ] Debugging?
- [ ] Something else I'm currently not aware of 😀