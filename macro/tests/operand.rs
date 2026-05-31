mod utility;

use melior::ir::{Block, Location, Type, ValueLike, block::BlockLike, operation::OperationLike};
use utility::*;

melior_macro::dialect! {
    name: "operand_test",
    files: ["macro/tests/ods_include/operand.td"],
}

#[test]
fn simple() {
    let context = create_test_context();
    context.set_allow_unregistered_dialects(true);

    let location = Location::unknown(&context);

    let r#type = Type::parse(&context, "i32").unwrap();
    let block = Block::new(&[(r#type, location), (r#type, location)]);
    let operation = operand_test::simple(
        &context,
        r#type,
        block.argument(0).unwrap().into(),
        block.argument(1).unwrap().into(),
        location,
    );

    assert_eq!(operation.lhs().unwrap(), block.argument(0).unwrap().into());
    assert_eq!(operation.rhs().unwrap(), block.argument(1).unwrap().into());
    assert_eq!(operation.as_operation().operand_count(), 2);
    assert_eq!(operation.as_operation().result_count(), 1);
    assert_eq!(operation.res().unwrap().r#type(), r#type);
}

#[test]
fn variadic_after_single() {
    let context = create_test_context();
    context.set_allow_unregistered_dialects(true);

    let location = Location::unknown(&context);

    let r#type = Type::parse(&context, "i32").unwrap();
    let block = Block::new(&[(r#type, location), (r#type, location), (r#type, location)]);
    let operation = operand_test::variadic(
        &context,
        r#type,
        block.argument(0).unwrap().into(),
        &[
            block.argument(2).unwrap().into(),
            block.argument(1).unwrap().into(),
        ],
        location,
    );

    assert_eq!(
        operation.first().unwrap(),
        block.argument(0).unwrap().into()
    );
    assert_eq!(
        operation.others().next(),
        Some(block.argument(2).unwrap().into())
    );
    assert_eq!(
        operation.others().nth(1),
        Some(block.argument(1).unwrap().into())
    );
    assert_eq!(operation.as_operation().operand_count(), 3);
    assert_eq!(operation.others().count(), 2);
    assert_eq!(operation.as_operation().result_count(), 1);
    assert_eq!(operation.res().unwrap().r#type(), r#type);
}

// `AttrSizedOperandSegments` builder codegen (the `operand_segment_sizes`
// accumulator, per-group setter writes, and the synthesis in `build()`) is
// exercised at compile time by generating `OperandTest_AttrSizedOp` and
// constructing its builder below. The full build()/accessor round-trip is not
// asserted here because it calls `set_inherent_attribute`, which requires a
// *registered* dialect — the `operand_test` dialect is unregistered. That path is
// covered end-to-end downstream by solx, whose sol dialect is registered and whose
// `sol.encode`/`sol.create` lowerings rely on the synthesized attribute.
#[test]
fn attr_sized_operand_segments_builder_compiles() {
    let context = create_test_context();
    let location = Location::unknown(&context);
    let _builder = operand_test::AttrSizedOperation::builder(&context, location);
}
