# Trait for slice/array

How to check slice/array implements something at compile time?
Write a function returning true.

```rust
trait Trait1 {}
impl< T : Sized > Trait1 for &[ T ] {}
fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
let src : &[ i32 ] = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &src ), true );
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=807aabe00f97c3b184da78379720d5b8)

That works, but replacing a variable by immidiate data breaks the program:

```rust
trait Trait1 {}
impl< T : Sized > Trait1 for &[ T ] {}
fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
let src = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &src ), true );
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=68823817984d1c9f94ae1bc713b74974)

Problem:

```log
  |
7 |   assert_eq!( does_implement_trait1( &src ), true );
  |               ---------------------  ^^^^ the trait `Trait1` is not implemented for `&[{integer}; 3]`
  |               |
  |               required by a bound introduced by this call
  |
```

But why? Let's check the type of src in both cases as well as its size:

```rust
let src1 : &[ i32 ] = &[ 1, 2, 3 ];
dbg!( &std::any::type_name_of_val( src1 ) );
dbg!( std::mem::size_of_val( &src1 ) );

let src2 = &[ 1, 2, 3 ];
dbg!( &std::any::type_name_of_val( src2 ) );
dbg!( std::mem::size_of_val( &src2 ) );

dbg!( std::mem::size_of_val( &1_usize ) );
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=4b7ae7def867dac6a03ea5a502544f05)

Output:
```log
[src/main.rs:6] &std::any::type_name_of_val(src1) = "[i32]"
[src/main.rs:7] std::mem::size_of_val(&src1) = 16
[src/main.rs:10] &std::any::type_name_of_val(src2) = "[i32; 3]"
[src/main.rs:11] std::mem::size_of_val(&src2) = 8
[src/main.rs:13] std::mem::size_of_val(&1_usize) = 8
```

What to conclude from the data?

- There are 2 distinct types: tick pointer with size in the first case and thin pointer in the second case.
- Probably, implementing a trait for one type of reference does not implement it for another one?

Let's apply our insights:

```rust
trait Trait1 {}
fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
impl< T : Sized > Trait1 for &[ T ] {}
impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
let src1 : &[ i32 ] = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &src1 ), true );
let src2 = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &src2 ), true );
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=5d346aca864d987e122468807c38800a)

Now trait Trait1 is implemented for both types of references.

Extra problem: let's fix the original problem so, that we get a thin pointer out of the array.
To solve the extra problem loosen prototype of function `does_implement_trait1` to make it accept unsized input adding `+ ?Sized` and add replace `&src2` by `&&src2[ .. ]`.

```rust
trait Trait1 {}
fn does_implement_trait1( _ : &( impl Trait1 + ?Sized ) ) -> bool { true }
impl< T : Sized > Trait1 for &[ T ] {}
// impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
let src1 : &[ i32 ] = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &src1 ), true );
let src2 = &[ 1, 2, 3 ];
assert_eq!( does_implement_trait1( &&src2[ .. ] ), true );
```

[Code](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b993179e3d69a19a619e20bd3ad76c16)

That runs.

### Conclusion

- There are 2 distinct types: tick pointer with size in the first case and thin pointer in the second case.
- Implementing trait for reference on array does not imply implementing the trait for a slice and vice versa.
