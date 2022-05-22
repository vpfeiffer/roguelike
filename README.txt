To run formatter:
`cargo fmt`

To run linter:
`cargo clippy`

To automatically create documentation:
`cargo doc --open`
Add document comments by using
`//!` or `///`
i.e.
```
/// Returns a string that is the name of a dog
///
/// # Arguments
/// 
/// * `name` - A string slice with the name of the dog
///
/// # Examples
/// 
/// ```
/// // this is a comment
/// use doc::Dog;
/// let dog = Dog::new("name");
/// ```
```
