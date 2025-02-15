# Off-by-one Error in Rust Vector Iteration

This repository demonstrates a common off-by-one error in Rust when removing elements from a vector while iterating over it using an index.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file provides a corrected version.

The issue arises because removing an element shifts subsequent elements, potentially causing the index to skip elements or go out of bounds, leading to unexpected behavior or panics.

The solution illustrates safer ways to handle this type of iteration and removal.