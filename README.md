# libloading-rs

## 1. This works

The following command runs correctly to completion:

```
cargo test -- --nocapture
```

The test on the `user` lib prints the following:

```
running 1 test
1. Spawn on current runtime from main lib
2. Enter say_hello, let's spawn a new task on the handle from the dylib...
3. Print something (will never happen)!
4. Return value
5. End of test
test tests::check_working_from_main_lib ... ok
```

## 2. This does not work

```
cargo run
```

The output is:

```
1. Spawn on current runtime from main lib
2. Enter say_hello, let's spawn a new task on the handle from the dylib...

```

Then the program halts indefinitely!
