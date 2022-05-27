mutable -> exterior reference

https://doc.rust-lang.org/std/cell/


shared reference -> but is allow to mutated -> control fashion
--> interior mutability, in the exterior is inmut


Cell -> no reference to what's inside a cell, It can be replaced, changed, copy, but not a pointer/reference inside a cell.
-> is always safe to mut for the first reason.
-> multiple shared ref to a cell. -> but is sigle threaded, one ref at a time. No support for multithreading. -> sync
-> get the values -> only through copy

