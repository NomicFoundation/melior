//! Downstream verification harness for solx-driven melior changes.
//!
//! Run with the solx LLVM-fork prefix in the environment:
//! `MLIR_SYS_210_PREFIX=.../target-final cargo run -p melior-verify`.

use melior::{
    Context,
    ir::{attribute::IntegerAttribute, r#type::IntegerType},
};

fn main() {
    let context = Context::new();

    verify_from_words(&context);

    println!("melior-verify OK");
}

/// A1 — `IntegerAttribute::from_words`: little-endian word constructor for
/// integers wider than the 64-bit `IntegerAttribute::new` ceiling.
fn verify_from_words(context: &Context) {
    // Single word round-trips like `new` (signless type, read via `value`).
    let i64_type = IntegerType::new(context, 64).into();
    assert_eq!(
        IntegerAttribute::from_words(i64_type, &[42]).value(),
        42,
        "from_words single word"
    );

    // Wide value: 2^64 in an i256 (4 little-endian words, second word = 1).
    let i256_type = IntegerType::new(context, 256).into();
    let wide = IntegerAttribute::from_words(i256_type, &[0, 1, 0, 0]);
    let printed = wide.to_string();
    assert!(printed.contains("18446744073709551616"), "wide value: {printed}");
    assert!(printed.contains("i256"), "wide type: {printed}");

    println!("  from_words: ok");
}
