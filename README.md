# Ray Tracer

A toy ray tracer (path tracer) written in Rust.

This follows the tutorial given by Peter Shirley's "Ray Tracing in One Weekend"
(v1.54, 2018) [1].

## Development

The main branch is `master`: should compile and run.

Any feature branches should have their commit history be properly groomed and
squashed into one commit so it can be rebased on top of `master`.

## Architecture Considerations

Based on the article by Dawid Ciezarkiewicz [2].

### Clean Architecture

Based on the article by Robert C. Martin [3].

Core objectives:

- **Separation of concerns**
- **Easy to test**
- **Easy to substitute details**

#### Dependency

That source code dependencies need to point from lower-level details to
higher-level abstractions, and not the other way around. Data structures,
formats, implementation details from lower-level details should *not* pollute
the design of higher-level abstractions.

The core rationale for this is that lower-level details are subject to frequent
changes and freqent breaking changes, while higher-level abstractions tend to be
more stable. A migration from MySQL to PostgreSQL is a low-level implementation
detail that should not affect nor infect the higher-level abstractions.

#### Clear Boundaries, and Crossing Boundaries

The *flow of control* need not be in the same direction as the *source code
dependency*. If the *flow of control* goes from higher-level abstractions to
lower-level details, then mitigation strategies can be employed to satisfy the
Dependecy Inversion Principle, that higher-level abstractions should *not*
depend on lower-level details.

- The mitigation strategy in Java is to use `interface`s, and we can do the same
  with Rust `trait`s - the abstraction of *what* the lower-level, and not *how*
  the lower-level module goes on to achieve it.
	* The `trait` must be declared in the higher-level module, so the direction
	  of source code dependency must come from lower-level module and to the
	  higher-level module.

For example, our application might need to persist some data, but the core logic
and domain concepts does not need to know and does not care how the persistence
is done – it might be via flat files, via MySQL, via distributed databases, or
even via in-memory databases – it does not matter and should not matter.

When we do need to cross from higher-level abstractions to lower-level details,
due to source code dependency, we typically should pass simple data structures
(known as *Data Transfer Objects* (DTO)) to the lower-level details. Note that
the declaration of the DTO **must** be within the higher-level abstractions as
to not reverse the direction of source code dependency.

## Other Design Considerations

### Should Builders Be Used?

When some function needs many arguments, it's good practice to encapsulate them
into a `struct`, especially if some of the arguments are *optional* and can have
*default* values. The *Builder* pattern comes into mind, just like many other
languages.

An important consideration here is for *visibility*. While the parent `struct`
may be declared `pub`, the `struct` may contain *crate-private* (`pub(crate)`)
or *private* members.

#### Possible Alternative: The "Init-Struct" Pattern?

This means that while `std::default::Default` is really useful for initializing
a `struct` with no *private* or *crate-private* members, it is may not be
suitable for `struct`s that have non-`pub` members in the case that such members
may not have a sensible default. In other words, it breaks encapsulation and can
cause unintended API breaking changes. Additionally, trying to use the `struct`
definition itself to initialize the data type can also leak implementation
detail (the "init-`struct`" pattern).

Consider the `std::process::Command` `struct`:

```rust
Command::new("cmd-name")
	.arg("foo")
	.arg("bar")
```

If the standard library authors were to use the init-`struct` pattern, it would
leak the implementation detail of how multiple `arg`s are handled.

```rust
Command {
	program: "cmd-name",
	args: vec![
		"foo",
		"bar"
	],
	..std::default::Default()
}
```

The underlying data structure for storing multiple `arg`s does not necessarily
*need* to be `std::vec::Vec`, but to construct the `struct` by initialization,
the actual underlying type of the `struct` member must be exposed as part of the
public API which is undesirable.

Another important consideration is that the `struct` requires each member have a
"final" type – you can't specify that some member can be of a size that is
convertible `Into<T>` for some target type `T`, for example. On the contrary,
builder methods *can* express this by specifying the argument be of type
`V: Into<T>` so as long as the provided source type of the argument is
convertible `Into` the desired target type `T`, which is the actual type of the
member of the `struct`.

```rust
// struct
pub struct Foo {
	bar: u32,
}

// builder
impl Foo {
// ...omitted
	fn bar<N: Into<u32>>(&mut self, n: N) -> &mut Self {
		// ...omitted
	}
// ...omitted
}
```

Based on discussions from
[r/rust](https://www.reddit.com/r/rust/comments/fg6vrn/for_your_consideration_an_alternative_to_the/).

#### Generate Builders with Procedural Macros or Hand-Roll Them?

There many quite a few crates for generating builders at compile-time through
clever procedural macro usage:
[idanarye/rust-typed-builder](https://github.com/idanarye/rust-typed-builder)
and
[colin-kiegel/rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder).

- `rust-typed-builder`:
	- Pros:
		* Does compile-time checks on builder internal state before `.build()`
		  is to avoid possibility of partial state and having to return
		  `Result` as the return type of `.build()`.
		* Does compile-time checks that no field can be set more than once.
	- Cons:
		* Poor error reporting.
		* `.build()` return type being non-`Result` causes another issue – it
		  may be the case that the members have some more specific/exotic
		  requirements/constraints on what values can occupy the given member
		  type.
		* The check that no field can be set more than once `<=` no builder
		  method can be invoked more than once constrains possibly useful
		  builder usages such as `std::process::Command::arg`.
- `rust-derive-builder`:
	- Pros:
		* `.build()` returns `Result` allows the expression of possible
		  construction failure.
	- Cons:
		* The procedural macro generation currently does not allow the user to
		  specify a custom `Error` type for the `.build()`'s `Result` type that
		  better suits the domain and is more expressive. As of this writing,
		  the return type of `.build()` is hard-coded to the
		  `Result<_, String>` type where the value type `_` is just whatever the
		  `struct` is, and the error type being hard-coded into a `String`,
		  which can cause problems with error-handling, such as the use of the
		  `?` operator or when trying to compose multiple `Result`s.

The question that whether a crate should be used to simplify the implementation
of the builder pattern by generation, then, seems to be answered by:

> Use a crate to generate/derive a builder impl for the struct IF AND ONLY IF
> the procedural macro is sufficiently powerful to address the specific use
> case, and should be *avoided* when the generated builders are simply not
> expressive enough to express all of the intents of your domain.
>
> This is especially the case if there are more complex run-time checks to be
> done when `.build()` is invoked.

## References and Links

1. https://github.com/petershirley/raytracinginoneweekend
2. https://dpc.pw/how-i-structure-my-apps-in-rust-and-other-languages
3. https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
