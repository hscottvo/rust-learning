# rust-learning
## Foreword
* I haven't been writing notes until I got to the slices section, so these notes are pretty incomplete, especially since they are catered to my experience going through the book.
## Slices
### Roadblocks
* remembering to use a reference when using `.iter()`
* ends of expressions should not have semicolons
    * this includes return statements in functions

### Notes
* The return statement in first_word has the `&` because it is returning a slice, which is a reference
* `String`s can't be iterated over by themselves, since it is ambiguous if you want to loop over the *bytes* versus the *characters*
  * If you do `my_string.bytes()` or `my_string.chars()` or set `my_string.to_bytes()` to a new character, the ambiguity is taken away and you can iterate over it like any other iterable
* byte strings are similar to python: `b' '` gives ASCII for space
* string literals are string slices