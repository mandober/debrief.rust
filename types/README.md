# Rust data types

* Scalar types

* Compound types

* Pointer types
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
