# rust-learning
## 5.1 Defining and Instantiating Structs
### Creating Instances
* using struct update syntax (copying fields from an existing instance by using `..struct_1_name` in the new struct) is an *assignment*
    * Depending on the fields copied, it can be a *move* or a *copy*. 
    * If __all__ fields taken from the first instance are *copy*-able, then the new instance does not invalidate the first. You can still read the first instance.
    * Otherwise, it was a *move* and you can't access the first instance.
### Tuple Structs without Named Fields
* used for having separate types of tuples without needing to enter in field names
* For example, in RGB, `(0, 0, 0)` is black, but in coordinate systems, it is the origin.
* Otherwise, acts like a normal tuple
### Fieldless Structs
* Implement traits on a type without adding the trait to the struct itself
* Covered in more detailed along with traits in chapter 10
### Ownership
* Earlier example had structs having `String` fields instead of `&str` fields
* If a struct does not own a field, those fields must have *lifetimes* (Chapter 10)
## 5.2 Rectangle Example
* not printable by default
* implement `Display()` in the struct or use `Debug()`
    * if `Debug()` isn't implemented either, can use `#[derive(Debug)]` above struct definition
    * can use `"{:?}"` to add to a string or `"{:#?}"` to have it pretty print
    * can use `dbg!` macro instead of `println!` to get more information, but uses `stderr` instead of `stdout`
## 5.3 Method Syntax
* methods are defined in the context of a struct unlike functions 
* first parameter is always self
### Defining Methods
* use `impl struct_name` block to implement methods for a particular struct
* methods are called using dot notation: `struct_name.method_name()`
* ownership can be moved around even with methods, so pay attention to using references etc
    * not using a reference (just using `self`) is rare, usually for transformation and not being able to use the original version anymore
### Methods with More Parameters
* more parameters are pretty much like in functions
* stay wary of ownership 
### Associated Functions
* all functions inside of an `impl` block is an *associated functions*
* functions that don't use `self` in their parameter list
    * eg. `String::from()` doesn't use `self` at all, just needs a string literal
* often used in constructors to return a new instance of a struct
* call associated functions with `::`
    * eg. `let square_1 = Rectangle::square(10);`
* can have multiple `impl` blocks per struct
    * reasoning in Chapter 10 for generic types and traits


