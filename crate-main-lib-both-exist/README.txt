[use crate behavior when both lib.rs and main.rs exists · Issue #2623 · rust-lang/book](https://github.com/rust-lang/book/issues/2623)
----
[https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary

It might be confusing for beginners to wonder what is the behavior of use crate::xxx when there are both main.rs and lib.rs in a single crate.

https://github.com/rust-lang/book/blob/master/src/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
https://github.com/rust-lang/reference/blob/master/src/items/use-declarations.md

I don't believe we discussed the behavior of use crate::xxx when there are both main.rs and lib.rs at least I haven't came across it in the docs. Not sure if the compiler warns about the usage of incorrect use crate::xxx, if it does not maybe we should open an issue in rust repo as well?](https://github.com/rust-lang/book/issues/2623)
----
[In chapter 7-1, we discuss the concept of a "crate root" and in relation to library and binary crates:

Looking at the contents of Cargo.toml, there’s no mention of src/main.rs because Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
Then in chapter 7-2, we discuss what crate means in relation to the concept of "crate root":

Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.
It's not incorrect to use use crate:: in either src/lib.rs or src/main.rs.

Do you have any suggestions on how we could make the information present in Chapter 7 more obvious or clear?](https://github.com/rust-lang/book/issues/2623)
----
I was just about to open a new issue for this same topic before finding this one!

After re-reading section 7-1 it does explain that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and that if both src/main.rs and src/lib.rs are present, they're compiled as two separate crates, but it didn't really stick with me the first time how to use the package name to use a struct or function defined in src/lib.rs from src/main.rs and I had similar confusion to https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs, trying crate:: paths before figuring it out. (I separately opened rust-lang/async-book#136 to help avoid the confusion I stumbled on.)

I think adding an example of use with a package name path in main.rs to use a pub struct or function defined in lib.rs would definitely help clarify this!

