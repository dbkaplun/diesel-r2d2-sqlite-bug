### The problem

```rust
    diesel::r2d2::Pool::builder()
        // .max_size(1) // uncommenting makes it work
        .build(manager)
```
[src/lib.rs#L36](https://github.com/dbkaplun/diesel-r2d2-sqlite-bug/blob/master/src/lib.rs#L36)

### To reproduce

```sh
$ RUST_BACKTRACE=1 cargo test
```

### Output

```rust
   Compiling diesel-r2d2-sqlite-bug v0.1.0 (file:///Users/dbkaplun/stuff/diesel-r2d2-sqlite-bug)
    Finished dev [unoptimized + debuginfo] target(s) in 4.67 secs
     Running target/debug/deps/diesel_r2d2_sqlite_bug-008b50899c57f975

running 1 test
Running migration 2018-03-30-164928_init
test test_connection ... FAILED

failures:

---- test_connection stdout ----
	Populating
thread 'test_connection' panicked at 'called `Result::unwrap()` on an `Err` value: DatabaseError(__Unknown, "database is locked")', libcore/result.rs:945:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:206
   3: std::panicking::default_hook
             at libstd/panicking.rs:216
   4: std::panicking::begin_panic
             at libstd/panicking.rs:400
   5: std::panicking::try::do_call
             at libstd/panicking.rs:347
   6: std::panicking::try::do_call
             at libstd/panicking.rs:323
   7: core::ptr::drop_in_place
             at libcore/panicking.rs:71
   8: core::result::unwrap_failed
             at /Users/travis/build/rust-lang/rust/src/libcore/macros.rs:26
   9: <core::result::Result<T, E>>::unwrap
             at /Users/travis/build/rust-lang/rust/src/libcore/result.rs:782
  10: diesel_r2d2_sqlite_bug::test_connection
             at src/lib.rs:21
  11: diesel_r2d2_sqlite_bug::__test::TESTS::{{closure}}
             at src/lib.rs:20
  12: core::ops::function::FnOnce::call_once
             at /Users/travis/build/rust-lang/rust/src/libcore/ops/function.rs:223
  13: <F as alloc::boxed::FnBox<A>>::call_box
             at libtest/lib.rs:1452
             at /Users/travis/build/rust-lang/rust/src/libcore/ops/function.rs:223
             at /Users/travis/build/rust-lang/rust/src/liballoc/boxed.rs:635
  14: panic_unwind::dwarf::eh::read_encoded_pointer
             at libpanic_unwind/lib.rs:101


failures:
    test_connection

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```
