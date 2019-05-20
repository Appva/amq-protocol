use crate::{
    flags::*,
    types::*,
    value::*,
};

use nom::{
    self,
    bytes::streaming::take,
    combinator::{all_consuming, complete, flat_map, map, map_opt, map_parser, map_res},
    error::ErrorKind,
    multi::fold_many0,
    number::streaming::{be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_i64, be_u64, be_f32, be_f64},
    sequence::pair,
};

/// Error returned by parsers
pub type ParserError<'a> = nom::Err<(&'a [u8], ErrorKind)>;
/// Return type of parsers
pub type ParserResult<'a, T> = Result<(&'a [u8], T), ParserError<'a>>;

/// Parse the [AMQPValue](../type.AMQPValue.html) of the given [AMQPType](../type.AMQPType.html)
pub fn parse_raw_value(amqp_type: AMQPType) -> impl Fn(&[u8]) -> ParserResult<'_, AMQPValue> {
    move |i| match amqp_type {
        AMQPType::Boolean        => map(parse_boolean,          AMQPValue::Boolean)(i),
        AMQPType::ShortShortInt  => map(parse_short_short_int,  AMQPValue::ShortShortInt)(i),
        AMQPType::ShortShortUInt => map(parse_short_short_uint, AMQPValue::ShortShortUInt)(i),
        AMQPType::ShortInt       => map(parse_short_int,        AMQPValue::ShortInt)(i),
        AMQPType::ShortUInt      => map(parse_short_uint,       AMQPValue::ShortUInt)(i),
        AMQPType::LongInt        => map(parse_long_int,         AMQPValue::LongInt)(i),
        AMQPType::LongUInt       => map(parse_long_uint,        AMQPValue::LongUInt)(i),
        AMQPType::LongLongInt    => map(parse_long_long_int,    AMQPValue::LongLongInt)(i),
        /* Rabbitmq treats LongLongUInt as a LongLongInt hence expose it as such */
        AMQPType::LongLongUInt   => map(parse_long_long_int,    AMQPValue::LongLongInt)(i),
        AMQPType::Float          => map(parse_float,            AMQPValue::Float)(i),
        AMQPType::Double         => map(parse_double,           AMQPValue::Double)(i),
        AMQPType::DecimalValue   => map(parse_decimal_value,    AMQPValue::DecimalValue)(i),
        AMQPType::ShortString    => map(parse_short_string,     AMQPValue::ShortString)(i),
        AMQPType::LongString     => map(parse_long_string,      AMQPValue::LongString)(i),
        AMQPType::FieldArray     => map(parse_field_array,      AMQPValue::FieldArray)(i),
        AMQPType::Timestamp      => map(parse_timestamp,        AMQPValue::Timestamp)(i),
        AMQPType::FieldTable     => map(parse_field_table,      AMQPValue::FieldTable)(i),
        AMQPType::ByteArray      => map(parse_byte_array,       AMQPValue::ByteArray)(i),
        AMQPType::Void           => Ok((i,                      AMQPValue::Void)),
    }
}

/// Parse an [AMQPValue](../type.AMQPValue.html)
pub fn parse_value(i: &[u8]) -> ParserResult<'_, AMQPValue> {
    flat_map(parse_type, parse_raw_value)(i)
}

/// Parse an [AMQPType](../type.AMQPType.html)
pub fn parse_type(i: &[u8]) -> ParserResult<'_, AMQPType> {
    map_opt(be_u8, |t| AMQPType::from_id(t as char))(i)
}

/// Parse an id [(ShortUInt)](../type.ShortUInt.html)
pub fn parse_id(i: &[u8]) -> ParserResult<'_, ShortUInt> {
    parse_short_uint(i)
}

/// Parse a [Boolean](../type.Boolean.html)
pub fn parse_boolean(i: &[u8]) -> ParserResult<'_, Boolean> {
    map(be_u8, |b| b != 0)(i)
}

/// Parse a [ShortShortInt](../type.ShortShortInt.html)
pub fn parse_short_short_int(i: &[u8]) -> ParserResult<'_, ShortShortInt> {
    be_i8(i)
}

/// Parse a [ShortShortUInt](../type.ShortShortUInt.html)
pub fn parse_short_short_uint(i: &[u8]) -> ParserResult<'_, ShortShortUInt> {
    be_u8(i)
}

/// Parse a [ShortInt](../type.ShortInt.html)
pub fn parse_short_int(i: &[u8]) -> ParserResult<'_, ShortInt> {
    be_i16(i)
}

/// Parse a [ShortUInt](../type.ShortUInt.html)
pub fn parse_short_uint(i: &[u8]) -> ParserResult<'_, ShortUInt> {
    be_u16(i)
}

/// Parse a [LongInt](../type.LongInt.html)
pub fn parse_long_int(i: &[u8]) -> ParserResult<'_, LongInt> {
    be_i32(i)
}

/// Parse a [LongUInt](../type.LongUInt.html)
pub fn parse_long_uint(i: &[u8]) -> ParserResult<'_, LongUInt> {
    be_u32(i)
}

/// Parse a [LongLongInt](../type.LongLongInt.html)
pub fn parse_long_long_int(i: &[u8]) -> ParserResult<'_, LongLongInt> {
    be_i64(i)
}

/// Parse a [LongLongUInt](../type.LongLongUInt.html)
pub fn parse_long_long_uint(i: &[u8]) -> ParserResult<'_, LongLongUInt> {
    be_u64(i)
}

/// Parse a [Float](../type.Float.html)
pub fn parse_float(i: &[u8]) -> ParserResult<'_, Float> {
    be_f32(i)
}

/// Parse a [Double](../type.Double.html)
pub fn parse_double(i: &[u8]) -> ParserResult<'_, Double> {
    be_f64(i)
}

/// Parse a [DecimalValue](../type.DecimalValue.html)
pub fn parse_decimal_value(i: &[u8]) -> ParserResult<'_, DecimalValue> {
    flat_map(parse_short_short_uint, |scale| map(parse_long_uint, move |value| DecimalValue { scale, value, }))(i)
}

/// Parse a [ShortString](../type.ShortString.html)
pub fn parse_short_string(i: &[u8]) -> ParserResult<'_, ShortString> {
    map(map(map_res(flat_map(parse_short_short_uint, take), std::str::from_utf8), ToString::to_string), ShortString)(i)
}

/// Parse a [LongString](../type.LongString.html)
pub fn parse_long_string(i: &[u8]) -> ParserResult<'_, LongString> {
    map(map(map_res(flat_map(parse_long_uint, take), std::str::from_utf8), ToString::to_string), LongString)(i)
}

/// Parse a [FieldArray](../type.FieldArray.html)
pub fn parse_field_array(i: &[u8]) -> ParserResult<'_, FieldArray> {
    map_parser(flat_map(parse_long_uint, take), all_consuming(fold_many0(complete(parse_value), FieldArray::default(), |mut acc, elem| {
        acc.0.push(elem);
        acc
    })))(i)
}

/// Parse a [Timestamp](../type.Timestamp.html)
pub fn parse_timestamp(i: &[u8]) -> ParserResult<'_, Timestamp> {
    parse_long_long_uint(i)
}

/// Parse a [FieldTable](../type.FieldTable.html)
pub fn parse_field_table(i: &[u8]) -> ParserResult<'_, FieldTable> {
    map_parser(flat_map(parse_long_uint, take), all_consuming(fold_many0(complete(pair(parse_short_string, parse_value)), FieldTable::default(), |mut acc, (key, value)| {
        acc.0.insert(key, value);
        acc
    })))(i)
}

/// Parse a [ByteArray](../type.ByteArray.html)
pub fn parse_byte_array(i: &[u8]) -> ParserResult<'_, ByteArray> {
    map(flat_map(parse_long_uint, take), |a| ByteArray(a.to_vec()))(i)
}

/// Parse the [AMQPFlags](../type.AMQPFlags.html) for which the names are provided
pub fn parse_flags<'a, 'b>(i: &'a [u8], names: &'b [&'b str]) -> ParserResult<'a, AMQPFlags> {
    map(take((names.len() + 7) / 8), |b| AMQPFlags::from_bytes(names, b))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value(&[84, 42, 42, 42, 42, 42,  42,  42,  42]),  Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266))));
        assert_eq!(parse_value(&[83, 0,  0,  0,  4,  116, 101, 115, 116]), Ok((EMPTY, AMQPValue::LongString(LongString("test".to_string())))));
    }

    #[test]
    fn test_parse_raw_value() {
        assert_eq!(parse_raw_value(AMQPType::Timestamp)   (&[42, 42, 42, 42, 42,  42,  42,  42]),  Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266))));
        assert_eq!(parse_raw_value(AMQPType::LongString)  (&[0,  0,  0,  4,  116, 101, 115, 116]), Ok((EMPTY, AMQPValue::LongString(LongString("test".to_string())))));
        /* Test internal exceptions */
        assert_eq!(parse_raw_value(AMQPType::LongLongUInt)(&[42, 42, 42, 42, 42,  42,  42,  42]),  Ok((EMPTY, AMQPValue::LongLongInt(3038287259199220266))));
        assert_eq!(parse_raw_value(AMQPType::ShortString) (&[4,  116, 101, 115, 116]),             Ok((EMPTY, AMQPValue::ShortString(ShortString("test".to_string())))));
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(&[116]), Ok((EMPTY, AMQPType::Boolean)));
        assert_eq!(parse_type(&[102]), Ok((EMPTY, AMQPType::Float)));
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_id(&[255, 255]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_boolean(&[0]), Ok((EMPTY, false)));
        assert_eq!(parse_boolean(&[1]), Ok((EMPTY, true)));
    }

    #[test]
    fn test_parse_short_short_int() {
        assert_eq!(parse_short_short_int(&[0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_int(&[255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_short_uint() {
        assert_eq!(parse_short_short_uint(&[0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_uint(&[255]), Ok((EMPTY, 255)));
    }

    #[test]
    fn test_parse_short_int() {
        assert_eq!(parse_short_int(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_int(&[255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_uint() {
        assert_eq!(parse_short_uint(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_uint(&[255, 255]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_long_int() {
        assert_eq!(parse_long_int(&[0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_int(&[255, 255, 255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_long_uint() {
        assert_eq!(parse_long_uint(&[0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_uint(&[255, 255, 255, 255]), Ok((EMPTY, 4294967295)));
    }

    #[test]
    fn test_parse_long_long_int() {
        assert_eq!(parse_long_long_int(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_long_int(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_long_long_uint() {
        assert_eq!(parse_long_long_uint(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_long_uint(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, 18446744073709551615)));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float(&[0,  0,  0,   0]),  Ok((EMPTY, 0.)));
        assert_eq!(parse_float(&[66, 41, 174, 20]), Ok((EMPTY, 42.42)));
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(parse_double(&[0,  0,  0,  0,   0,   0,  0,  0]),   Ok((EMPTY, 0.)));
        assert_eq!(parse_double(&[64, 69, 53, 194, 143, 92, 40, 246]), Ok((EMPTY, 42.42)));
    }

    #[test]
    fn test_parse_decimal_value() {
        assert_eq!(parse_decimal_value(&[0,   0,   0,   0,   0]),   Ok((EMPTY, DecimalValue { scale: 0,   value: 0          })));
        assert_eq!(parse_decimal_value(&[255, 255, 255, 255, 255]), Ok((EMPTY, DecimalValue { scale: 255, value: 4294967295 })));
    }

    #[test]
    fn test_parse_short_string() {
        assert_eq!(parse_short_string(&[0]),                     Ok((EMPTY, ShortString::default())));
        assert_eq!(parse_short_string(&[4, 116, 101, 115, 116]), Ok((EMPTY, ShortString("test".to_string()))));
    }

    #[test]
    fn test_parse_long_string() {
        assert_eq!(parse_long_string(&[0, 0, 0, 0]),                     Ok((EMPTY, LongString::default())));
        assert_eq!(parse_long_string(&[0, 0, 0, 4, 116, 101, 115, 116]), Ok((EMPTY, LongString("test".to_string()))));
    }

    #[test]
    fn test_parse_field_array() {
        assert_eq!(parse_field_array(&[0, 0, 0, 0]),                                          Ok((EMPTY, FieldArray::default())));
        assert_eq!(parse_field_array(&[0, 0, 0, 10, 83, 0, 0, 0, 4, 116, 101, 115, 116, 86]), Ok((EMPTY, FieldArray(vec![AMQPValue::LongString(LongString("test".to_string())), AMQPValue::Void]))));
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_timestamp(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, 18446744073709551615)));
    }

    #[test]
    fn test_parse_field_table() {
        let mut table = FieldTable::default();
        table.0.insert(ShortString("test".to_string()), AMQPValue::LongString(LongString("test".to_string())));
        table.0.insert(ShortString("tt".to_string()),   AMQPValue::Void);
        assert_eq!(parse_field_table(&[0, 0, 0, 0]),                                                                              Ok((EMPTY, FieldTable::default())));
        assert_eq!(parse_field_table(&[0, 0, 0, 18, 4, 116, 101, 115, 116, 83, 0, 0, 0, 4, 116, 101, 115, 116, 2, 116, 116, 86]), Ok((EMPTY, table)));
    }

    #[test]
    fn test_parse_byte_array() {
        assert_eq!(parse_byte_array(&[0, 0, 0, 0]),              Ok((EMPTY, ByteArray::default())));
        assert_eq!(parse_byte_array(&[0, 0, 0, 4, 42, 1, 2, 3]), Ok((EMPTY, ByteArray(vec![42, 1, 2, 3]))));
    }

    #[test]
    fn test_parse_flags() {
        let mut flags = AMQPFlags::default();
        let mut names = Vec::new();
        names.push("a"); flags.add_flag("a".to_string(), true);
        names.push("b"); flags.add_flag("b".to_string(), false);
        names.push("c"); flags.add_flag("c".to_string(), true);
        names.push("d"); flags.add_flag("d".to_string(), true);
        assert_eq!(parse_flags(&[0b00001101], &names), Ok((EMPTY, flags.clone())));
        names.push("e"); flags.add_flag("e".to_string(), true);
        names.push("f"); flags.add_flag("f".to_string(), false);
        names.push("g"); flags.add_flag("g".to_string(), true);
        names.push("h"); flags.add_flag("h".to_string(), true);
        names.push("i"); flags.add_flag("i".to_string(), false);
        names.push("j"); flags.add_flag("j".to_string(), true);
        assert_eq!(parse_flags(&[0b11011101, 0b00000010], &names), Ok((EMPTY, flags)));
    }
}
