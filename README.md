# Clippy bug `single_char_pattern`

This repo showcases a bug in Clippy, specifically the `single_char_pattern` lint. See rust-lang/rust-clippy#14893. To reproduce the issue:

1. Clone the repository.
2. This bug is confirmed on Rust toolchain `1.89.0-nightly (68db37499 2025-05-22)`, so use this version, or the bug might have been fixed. Replace `+nightly` in following commands as appropriate.
3. Run `cargo +nightly check` - the code should pass the check.
4. Run `cargo +nightly clippy --fix` - you'd get `failed to automatically apply fixes`, as documented below.

```shell
$ cargo +nightly clippy --fix
    Checking clippy-bug-single_char_pattern v0.1.0 (/home/ubuntu/clippy-bug-single_char_pattern)
warning: failed to automatically apply fixes suggested by rustc to crate `clippy_bug_single_char_pattern`

after fixes were automatically applied the compiler reported errors within these files:

  * src/main.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust-clippy/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
error[E0308]: mismatched types
  --> src/main.rs:15:49
   |
15 |     let mut parts: Split<'_, &str> = time.split(':');
   |                                           ----- ^^^ expected `&str`, found `char`
   |                                           |
   |                                           arguments to this method are incorrect
   |
help: the return type of this call is `char` due to the type of the argument passed
  --> src/main.rs:15:38
   |
15 |     let mut parts: Split<'_, &str> = time.split(':');
   |                                      ^^^^^^^^^^^---^
   |                                                 |
   |                                                 this argument influences the return type of `split`
note: method defined here
  --> /rustc/5e16c662062fd6dee91f0fe2a1580483488d80cf/library/core/src/str/mod.rs:1605:12
help: if you meant to write a string literal, use double quotes
   |
15 -     let mut parts: Split<'_, &str> = time.split(':');
15 +     let mut parts: Split<'_, &str> = time.split(":");
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
Original diagnostics will follow.

warning: single-character string constant used as pattern
  --> src/main.rs:15:49
   |
15 |     let mut parts: Split<'_, &str> = time.split(":");
   |                                                 ^^^ help: consider using a `char`: `':'`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#single_char_pattern
note: the lint level is defined here
  --> src/main.rs:1:9
   |
1  | #![warn(clippy::single_char_pattern)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `clippy-bug-single_char_pattern` (bin "clippy-bug-single_char_pattern") generated 1 warning (run `cargo clippy --fix --bin "clippy-bug-single_char_pattern"` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
```
