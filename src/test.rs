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
        format!("#[derive(Debug)] struct TestStruct{{ test_u8: u8, test_vec: &\'static [&\'static str], }}")
    );
    assert_eq!
    (
        const_declaration!(TEST_STRUCT = test_struct),
        format!("const TEST_STRUCT: TestStruct = TestStruct {{ test_u8: 21u8, test_vec: &[\"Hello there.\"], }};")
    );
}

#[test]
fn test_struct_definition()
{
    assert_eq!
    (
        const_definition!(#[derive(Debug)] TestStruct), 
        format!("#[derive(Debug)] struct TestStruct{{ test_u8: u8, test_vec: &\'static [&\'static str], }}")
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
    assert_eq!
    (
        const_declaration!(TEST_TUP_STRUCT = test_tup_struct),
        format!("const TEST_TUP_STRUCT: TestTup = TestTup(4u8,55u16,);")
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
    assert_eq!
    (
        const_declaration!(TEST_ENUM = test_enum),
        format!("const TEST_ENUM: TestEnum = TestEnum::Variant1;")
    );

    let test_enum = TestEnum::Variant2(22);
    assert_eq!
    (
        const_declaration!(TEST_ENUM = test_enum),
        format!("const TEST_ENUM: TestEnum = TestEnum::Variant2(22u8,);")
    );

    let test_enum = TestEnum::Variant3 { named: 0 };
    assert_eq!
    (
        const_declaration!(TEST_ENUM = test_enum),
        format!("const TEST_ENUM: TestEnum = TestEnum::Variant3{{named:0u8,}};")
    );
}

#[test]
fn test_strings()
{
    assert_eq!
    (
        const_declaration!(pub(crate) TEST_STR = "I'm a string!"),
        format!("pub(crate)  const TEST_STR: &'static str = \"I'm a string!\";")
    );
    assert_eq!
    (
        const_declaration!(TEST_STRING = String::from("I'm a string!")),
        format!("const TEST_STRING: &'static str = \"I'm a string!\";")
    );
    assert_eq!
    (
        const_declaration!(TEST_COW = std::borrow::Cow::from("I'm a string!")),
        format!("const TEST_COW: &'static str = \"I'm a string!\";")
    );
}

#[test]
fn test_nums() 
{
    fn test<T: CompileConst + std::fmt::Display>(var_name: &str, type_name: &str, val: T)
    {
        assert_eq!
        (
            val.const_declaration("", "pub", var_name), 
            format!("pub const {0}: {1} = {2}{1};", var_name, type_name, val)
        );
    }
    test("TEST_U8", "u8", u8::MAX);
    test("TEST_U16", "u16", u16::MAX);
    test("TEST_U32", "u32", u32::MAX);
    test("TEST_U64", "u64", u64::MAX);
    test("TEST_U128", "u128", u128::MAX);
    test("TEST_USIZE", "usize", usize::MAX);
    test("TEST_I8", "i8", i8::MAX);
    test("TEST_I16", "i16", i16::MAX);
    test("TEST_I32", "i32", i32::MAX);
    test("TEST_I64", "i64", i64::MAX);
    test("TEST_I128", "i128", i128::MAX);
    test("TEST_F32", "f32", f32::MAX);
    test("TEST_F64", "f64", f64::MAX);
}


#[cfg(feature = "phf")]
#[test]
fn test_map()
{
    let mut test_map: HashMap<&str, i32> = HashMap::new();
    test_map.insert("str", 67);
    assert_eq!
    (
        const_declaration!(TEST_MAP = test_map),
        format!("const TEST_MAP: phf::Map<&\'static str, i32> = phf::phf_map!{{\"str\" => 67i32}};")
    );
}

#[cfg(feature = "phf")]
#[test]
fn test_set()
{
    let mut test_set: HashSet<i32> = HashSet::new();
    test_set.insert(34);
    assert_eq!
    (
        const_declaration!(TEST_SET = test_set),
        format!("const TEST_SET: phf::Set<i32> = phf::phf_set!{{34i32}};")
    );
}

#[test]
fn test_vec()
{
    let test_vec: Vec<u8> = vec!(1,2,3,4,5,10,4);
    assert_eq!
    (
        const_declaration!(TEST_VEC = test_vec),
        format!("const TEST_VEC: &'static [u8] = &[1u8,2u8,3u8,4u8,5u8,10u8,4u8];")
    );
}

#[test]
fn test_array()
{
    let arr: [u8; 0] = [];
    assert_eq!
    (
        const_declaration!(TEST_ARR = arr),
        format!("const TEST_ARR: [u8; 0] = [];")
    );
    let arr: [u8; 1] = [7];
    assert_eq!
    (
        const_declaration!(TEST_ARR = arr),
        format!("const TEST_ARR: [u8; 1] = [7u8];")
    );
    let arr: [u8; 9] = [1,2,3,4,5,6,7,8,9];
    assert_eq!
    (
        const_declaration!(TEST_ARR = arr),
        format!("const TEST_ARR: [u8; 9] = [1u8,2u8,3u8,4u8,5u8,6u8,7u8,8u8,9u8];")
    );
}

#[test]
fn test_const_array_strings()
{
    let s: &str = "Hello";
    assert_eq!
    (
        const_array_declaration!(TEST_CONST_STR = s),
        format!("const TEST_CONST_STR: [char; 5] = ['H','e','l','l','o',];")
    );
}

#[test]
fn test_const_array_slices()
{
    let test_enum: &'static [TestEnum] = &[TestEnum::Variant1, TestEnum::Variant2(7), TestEnum::Variant1];
    assert_eq!
    (
        const_array_declaration!(TEST_CONST_SLICE = test_enum),
        format!("const TEST_CONST_SLICE: [TestEnum; 3] = [TestEnum::Variant1,TestEnum::Variant2(7u8,),TestEnum::Variant1];")
    );
}

#[test]
fn test_const_array_derefs()
{
    let test_enum: Box<&'static [TestEnum]> = Box::new(&[TestEnum::Variant1, TestEnum::Variant2(7), TestEnum::Variant1]);
    assert_eq!
    (
        const_array_declaration!(TEST_CONST_SLICE = test_enum),
        format!("const TEST_CONST_SLICE: [TestEnum; 3] = [TestEnum::Variant1,TestEnum::Variant2(7u8,),TestEnum::Variant1];")
    );
}

#[test]
fn test_const_array_tuples()
{
    let test_enum: (&'static [TestEnum], Vec<i8>) = (&[TestEnum::Variant2(0)], vec!(1,2,3));
    assert_eq!
    (
        const_array_declaration!(TEST_CONST_TUP = test_enum),
        format!("const TEST_CONST_TUP: ([TestEnum; 1],[i8; 3]) = ([TestEnum::Variant2(0u8,)],[1i8,2i8,3i8]);")
    );
}