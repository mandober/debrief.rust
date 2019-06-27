# Rust-related Glossary

<!-- TOC -->

- [Blanket implementations](#blanket-implementations)
- [Bors](#bors)
- [Dynamically sized type](#dynamically-sized-type)
- [Fat pointer](#fat-pointer)
- [ICE](#ice)
- [Interior mutability](#interior-mutability)
- [Marker interfaces](#marker-interfaces)
- [Newtype](#newtype)
- [Phantom data](#phantom-data)
- [Phantom types](#phantom-types)
- [Raw pointer](#raw-pointer)
- [Rust](#rust)
- [Slice](#slice)
- [Trait object](#trait-object)
- [Zero sized type](#zero-sized-type)

<!-- /TOC -->


## Blanket Implementation
Conditionally implementing a trait that depends on a pre-existing implementation of another trait. Trait implementation on a type that already satisfies trait's bounds is called blanket implementation. For example, `ToString` trait in std is implemented conditionally on any type that implements `Display` trait (a type that can be printed is surely convertible to a string).

## Bors
Pet name for Rust's CI bot on GitHub. It's a script that runs tests on a, previously reviewed, pull request, merging it if all tests pass successfully.

## Dynamically Sized Type
Dynamically Sized Types (DST) do not have statically known size or alignment, so they only exist behind a fat (multi-worded) pointer. Common DST are *trait objects* and *slices*.

## Fat pointer
A fat pointer is a pointer augmented with extra information. The extra information takes the form of one or more associated fields that complement the pointer by providing additional info about the "pointed to" object. For example,`String` type is a data structure consisting of a pointer to the heap (pointing to some text data), augmented with the "length" and "capacity" fields that provide more info about the payload.

## ICE
Internal Compiler Error (ICE) is an assertion failure in the compiler code, indicating a bug.

## Interior mutability
Interior mutability is a strategy used to force mutation of an, otherwise immutable type, by exposing methods to mutate the type's payload (its inner value) in a manner that suspends compile-time borrow checking, by postponing it to the run-time.

## Marker interfaces
Marker interfaces contain no methods at all and serve to provide run-time information to generic processing using reflection. In Rust, marker interface is realized through marker traits: Copy, Sized, Send, Sync.

## Newtype
A tuple structure with a single unnamed field. Used to create type wrappers. For example: `struct Meter(i32)`. 

## Phantom data
Zero-sized type used to mark things that "act like" they own a `T`. Adding a `PhantomData<T>` field to your type tells the compiler that your type acts as though it stores a value of type `T`, even though it doesn't really. This information is used when computing certain safety properties. More about [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Phantom types
Phantom data and phantom types are related, but not identical. A phantom type parameter is simply a type parameter which is never used. In Rust, this often causes the compiler to complain, and the solution is to add a dummy use by way of [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Raw pointer
A pointer is a variable that contains the memory address of some value. To access the value it points to, the pointer is dereferenced. In Rust, these kind of pointers are called raw pointers; there is immutable raw pointer, `*const T` and mutable raw pointer, `*mut T`.

## Rust
According to one version, the Rust language gets its name after a fungi that is very robust, distributed (as opposed to a non-single-cellular), with a parallel reproduction. Now, hold the adjectives, drop the rest.

## Slice
A slice is a view into some contiguous storage. Slice is a fat pointer; the information that completes a slice is the number of elements it points to.

## Trait
A type class. Similar to an interface in other languages, prone to composition rather then inheritance.

## Trait object
Dynamically sized type that implements some trait. The original type is _erased_ in favor of runtime reflection, with a _vtable_ containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a pointer to its vtable.

## Zero sized type
ZST are types that occupy no space (e.g. empty tuple, empty array).
