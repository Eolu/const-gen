use std::{collections::{HashMap, HashSet}, u64};
use crate::{self as const_gen, *};

#[cfg(feature = "derive")]
#[derive(CompileConst)]
struct TestStruct
{
    test_u8: u8,
    test_vec: Vec<String>,
}

/// Will use the above struct to generate a struct like this:
/// 
/// struct TestStruct
/// {
///    test_u8: u8,
///    test_vec: &'static [&'static str],
/// }
#[test]
fn test_struct()
{
    let test_struct = TestStruct{test_u8: 21, test_vec: vec!(String::from("Hello there."))};
    assert_eq!
    (
        const_definition!(#[derive(Debug)] TestStruct), 
        format!("#[derive(Debug)]struct TestStruct{{ test_u8: u8, test_vec: &\'static [&\'static str], }}")
    );
    assert_eq!
    (
        test_struct.const_declaration("TEST_STRUCT"), 
        format!("const TEST_STRUCT: TestStruct = TestStruct {{ test_u8: 21u8, test_vec: &[\"Hello there.\"], }};")
    );
}

#[cfg(feature = "derive")]
#[derive(CompileConst)]
struct TestTup(u8, u16);

/// Will use the above struct to generate a struct like this:
/// 
/// struct TestTup(u8, u16);
#[test]
fn test_tup_struct()
{
    let test_tup_struct = TestTup(4, 55);
    println!
    (   
        "{}",
        test_tup_struct.const_declaration("TEST_TUP_STRUCT")
    );
}

#[cfg(feature = "derive")]
#[derive(CompileConst)]
enum TestEnum
{
    Variant1,
    Variant2(u8),
    Variant3 { named: u8 }
}

/// Will use the above enum to generate an enum like this:
/// 
/// enum TestEnum
/// {
///    Variant1,
///    Variant2(u8),
///    Variant3 { named: u8 }
/// }
#[test]
fn test_enum()
{
    let test_enum = TestEnum::Variant1;
    println!
    (
        "{}",
        test_enum.const_declaration("TEST_ENUM")
    );

    let test_enum = TestEnum::Variant2(22);
    println!
    (
        "{}",
        test_enum.const_declaration("TEST_ENUM")
    );

    let test_enum = TestEnum::Variant3 { named: 0 };
    println!
    (
        "{}",
        test_enum.const_declaration("TEST_ENUM")
    );
}

#[test]
fn test_strings()
{
    println!
    (
        "{}",
        "I'm a string!".const_declaration("TEST_STR")
    );
    println!
    (
        "{}",
        String::from("I'm a string!").const_declaration("TEST_STRING")
    );
    println!
    (
        "{}",
        std::borrow::Cow::from("I'm a string!").const_declaration("TEST_COW")
    );
}

#[test]
fn test_nums() 
{
    fn test<T: CompileConst + std::fmt::Display>(var_name: &str, val: T)
    {
        println!
        (
            "{}",
            val.const_declaration(var_name)
        );
    }
    test("TEST_U8", u8::MAX);
    test("TEST_U16", u16::MAX);
    test("TEST_U32", u32::MAX);
    test("TEST_U64", u64::MAX);
    test("TEST_U128", u128::MAX);
    test("TEST_USIZE", usize::MAX);
    test("TEST_I8", i8::MAX);
    test("TEST_I16", i16::MAX);
    test("TEST_I32", i32::MAX);
    test("TEST_I64", i64::MAX);
    test("TEST_I128", i128::MAX);
    test("TEST_F32", f32::MAX);
    test("TEST_F64", f64::MAX);
}


#[cfg(feature = "phf")]
#[test]
fn test_map()
{
    let mut test_map: HashMap<&str, i32> = HashMap::new();
    test_map.insert("str", 67);
    println!
    (
        "{}",
        test_map.const_declaration("TEST_MAP")
    );
}

#[cfg(feature = "phf")]
#[test]
fn test_set()
{
    let mut test_set: HashSet<i32> = HashSet::new();
    test_set.insert(34);
    println!
    (
        "{}",
        test_set.const_declaration("TEST_SET")
    );
}

#[test]
fn test_vec()
{
    let test_vec: Vec<u8> = vec!(1,2,3,4,5,10,4);
    println!
    (
        "{}",
        test_vec.const_declaration("TEST_VEC")
    );
}