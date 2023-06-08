<h1 align="center"><b> Notes </b></h1>

---

<h3 align="center"><b> Rust... </b></h3><br>

Incoming....

<br>

---

<h3 align="center"><b> 📝 Misc. Notes </b></h3>

| <s> | <s>
|--|--
| '' | denotes character type
| "" | denotes string type
| * | dereferences and yields an object (value, string, array / vector...)
| &  | for 'borrowing' a reference to an object, such as in loops
| ? | used on functions that can fail to provide the req Ok(s) / Err(e) flow
| Functions |  fallible functions in Rust should return a Result value, which is either Ok(s) on success, where s is the successful value, or Err(e) on failure, where e is an error code
| Booleans | strictly enforced : Strictly enforced, rust won't treat empty / non-empty as True / False, and loops must use full expressions
| Tuples | allow for mixed dtype collections, unlike arrays, and require constant values for indexing e.g. tup[0] is valid but tup[i] is not. Heavily used in function returns due to mixed dtype capacity
| Referencing | &T - immutable ref - can be used by any number of readers<br>&mut T - mutable ref - only a single read / write ref can be active at one time
| Arrays | [T; N] is an array of N values, of type T : let a: [u32; 3] = [1,2,3]; - or - let b: [u32; 3] = [0; 3] (three 0's)
| Vector | Vec<T> is a dynamically changable sequence
| Slices | &[T] and &mut [T] are shared slices - like a pointer to the first element with a count of the available entries
| Indexing | are available as : v[0] ... v[i] ... v[v.len() - 1]

<br>

---

<h3 align="center"><b> 🎹 Command Line </b></h3><br>

**Rust docs**

````ps1
# Open the rust docs
rustup doc

# Open the rust docs for a library
rustup doc --std

````

**Rust config commands**

Powershell scripts in [CMD](/cmd/) dir

````ps1
# Package manager version
cargo --version

# Toolchain manager
rustup --version

# Compiler version
rustc --version

# Documentation version
rustdoc --version

# Update rust
rustup update

````

**Cargo setup commands**

````ps1
# New project - with git initiation
cargo new <project_name>

# New project - without git initiation
cargo new <project_name> --vcs none

# Initialise a rust project in current dir
cargo init

````

**Cargo build commands (dir with cargo.toml)**

````ps1
# Run project
cargo run

# Build project 
cargo build

# Build release 
cargo build --release

# Clear build files
cargo clear

# Run project tests
cargo test

````

<br>

---
