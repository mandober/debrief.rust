# Type categorization by levels

- allocation
- alignment
- functionality
- atomicity
- mutability
- complexity, compound
- implementation
- representation
- constraints, type class
- algebraic level
- value level
- logical level (linear, affine, session, etc. types)
- user level (builtin vs user/custom types)
- structural (nominal, structural)



---

Allocation level:
- static types
- stack types
- heap types

Functional level
- builtin types
- DST
- ZST
- phantom types
- FFI
- foreign types
- opaque types

Construction level
- primitive
- compound types (from scalars)

Complexity
- trait types
- constrained types
- trait object types

Algebraic level
- sum types
- product types
- dependent types
- recursive types

Impl level
- compiler
- core
- library

Value level
- direct types
- pointer/reference types

Atomicity
- atomic
- non-atomic 

Constrained
- constrained types
- unconstrained types



## Pointer types
  - function pointer (stack pointer)
  * References
    - shared reference (pointer to stack)
    - mutable reference (unique pointer to stack)
  * Raw pointers
    - raw constant pointer (unsafe const pointer)
    - raw mutable pointer (unsafe mutable pointer)
  - slice (unsized, fat pointer, to array or vector)
  - str (unsized, fat pointer, to a stringy data)
  * Smart pointers (fat pointers to heap payload):
    - `Box`
    - `Vec`
    - `String
    - `Rc` (reference counting)
    - `Arc` (atomic reference counting)
    - trait object (function dispatcher)
* Collections
  - hashmap
