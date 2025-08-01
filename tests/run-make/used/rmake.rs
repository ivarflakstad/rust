//@ needs-target-std
//@ ignore-wasm (`object` can't handle wasm object files)

// This test ensures that the compiler is keeping static variables, even if not referenced
// by another part of the program, in the output object file.
//
// It comes from #39987 which implements this RFC for the #[used] attribute:
// https://rust-lang.github.io/rfcs/2386-used.html

use run_make_support::rustc;
use run_make_support::symbols::object_contains_any_symbol_substring;

fn main() {
    rustc().opt_level("3").emit("obj").input("used.rs").run();
    assert!(object_contains_any_symbol_substring("used.o", &["FOO"]));
}
