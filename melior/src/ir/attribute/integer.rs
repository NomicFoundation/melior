use super::{Attribute, AttributeLike};
use crate::{
    Error,
    ir::{Type, TypeLike},
};
use mlir_sys::{
    MlirAttribute, mlirIntegerAttrGet, mlirIntegerAttrGetFromWords, mlirIntegerAttrGetValueInt,
    mlirIntegerAttrGetValueSInt, mlirIntegerAttrGetValueUInt,
};

/// An integer attribute.
#[derive(Clone, Copy, Hash)]
pub struct IntegerAttribute<'c> {
    attribute: Attribute<'c>,
}

impl<'c> IntegerAttribute<'c> {
    /// Creates an integer attribute.
    pub fn new(r#type: Type<'c>, integer: i64) -> Self {
        unsafe { Self::from_raw(mlirIntegerAttrGet(r#type.to_raw(), integer)) }
    }

    /// Creates an integer attribute from little-endian 64-bit words.
    ///
    /// `words` holds the value in little-endian order (least-significant word
    /// first) and must contain exactly `ceil(bit_width / 64)` elements for the
    /// bit width of `r#type`. This is the constructor for integers wider than
    /// 64 bits (e.g. the 256-bit constants the EVM/Sol dialects use), which
    /// [`IntegerAttribute::new`] cannot express.
    pub fn from_words(r#type: Type<'c>, words: &[u64]) -> Self {
        unsafe {
            Self::from_raw(mlirIntegerAttrGetFromWords(
                r#type.to_raw(),
                u32::try_from(words.len()).expect("word count fits in u32"),
                words.as_ptr(),
            ))
        }
    }

    /// Returns a value.
    pub fn value(&self) -> i64 {
        unsafe { mlirIntegerAttrGetValueInt(self.to_raw()) }
    }

    /// Returns a value as a signed integer.
    pub fn signed_value(&self) -> i64 {
        unsafe { mlirIntegerAttrGetValueSInt(self.to_raw()) }
    }

    /// Returns a value as an unsigned integer.
    pub fn unsigned_value(&self) -> u64 {
        unsafe { mlirIntegerAttrGetValueUInt(self.to_raw()) }
    }
}

attribute_traits!(IntegerAttribute, is_integer, "integer");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ir::r#type::IntegerType, test::create_test_context};

    #[test]
    fn value() {
        let context = create_test_context();

        assert_eq!(
            IntegerAttribute::new(IntegerType::new(&context, 64).into(), 42).value(),
            42
        );
    }

    #[test]
    fn from_words_single() {
        let context = create_test_context();

        assert_eq!(
            IntegerAttribute::from_words(IntegerType::new(&context, 64).into(), &[42]).value(),
            42
        );
    }

    #[test]
    fn from_words_wide() {
        let context = create_test_context();

        // 2^64, little-endian words across an i256 (4 words).
        let attribute =
            IntegerAttribute::from_words(IntegerType::new(&context, 256).into(), &[0, 1, 0, 0]);

        assert!(attribute.is_integer());
        let printed = attribute.to_string();
        assert!(printed.contains("18446744073709551616"), "{printed}");
        assert!(printed.contains("i256"), "{printed}");
    }
}
