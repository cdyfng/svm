use std::convert::TryFrom;

use wasmer_runtime_core::types::Value as WasmerValue;

/// Wasm integer value
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Value {
    /// 32-bits
    I32(u32),

    /// 64-bits
    I64(u64),
}

/// Casting to a wasm integer value has failed
#[derive(Debug, PartialEq)]
pub enum ValueCastError {
    /// Not supported wasm primitive (i.e: floats, SIMD)
    NotSupportedType(&'static str),
}

impl ToString for ValueCastError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<&WasmerValue> for Value {
    type Error = ValueCastError;

    fn try_from(value: &WasmerValue) -> Result<Self, Self::Error> {
        match value {
            WasmerValue::I32(v) => Ok(Value::I32(*v as u32)),
            WasmerValue::I64(v) => Ok(Value::I64(*v as u64)),
            WasmerValue::F32(_) => Err(ValueCastError::NotSupportedType("F32")),
            WasmerValue::F64(_) => Err(ValueCastError::NotSupportedType("F64")),
            WasmerValue::V128(_) => Err(ValueCastError::NotSupportedType("V128")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_try_from() {
        let wasmer_value = WasmerValue::I32(10);
        assert_eq!(Ok(Value::I32(10)), Value::try_from(&wasmer_value));

        let wasmer_value = WasmerValue::I64(20);
        assert_eq!(Ok(Value::I64(20)), Value::try_from(&wasmer_value));

        let wasmer_value = WasmerValue::F32(10.0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("F32")),
            Value::try_from(&wasmer_value)
        );

        let wasmer_value = WasmerValue::F64(20.0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("F64")),
            Value::try_from(&wasmer_value)
        );

        let wasmer_value = WasmerValue::V128(0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("V128")),
            Value::try_from(&wasmer_value)
        );
    }
}
