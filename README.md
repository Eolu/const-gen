# const-gen

This is a crate for generating compile-time constants in your `build.rs` file. This crate supports converting types that are typically heap-allocated into fixed-size constants. It includes support for primitives, strings, vectors, maps, sets, and comes with a derive macro to allow implementation with structs and enums.

See this example:
```rust
// build.rs

use const_gen::*;
use std::{env, fs, path::Path};

// First, let's dummy up some structs. Enabling the "derive" 
// feature allows us to do this simply, but implementing the
// CompileConst trait by hand is straightforward.

#[derive(CompileConst)]
struct TestStruct
{
    test_u8: u8,
    test_vec: Vec<String>,
}

#[derive(CompileConst)]
struct TestTup(u8, u16);

#[derive(CompileConst)]
enum TestEnum
{
    Variant1,
    Variant2(u8),
    Variant3 { named: u8 }
}

fn main() 
{
    // Use the OUT_DIR environment variable to get an 
    // appropriate path.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");

    // Now let's dummy up some data to use in our const 
    // generation
    let test_vec: Vec<u8> = vec!(1,2,3,4,5,10,4);
    let test_struct = TestStruct
    { 
        test_u8: 12, 
        test_vec: vec!(String::from("Hello there.")) 
    };
    let test_tup_struct = TestTup(4, 55,);
    let test_enum = TestEnum::Variant1;
    let test_enum_tup = TestEnum::Variant2(23);
    let test_enum_structlike = TestEnum::Variant3{ named: 78 };

    // Now we'll generate the const declarations. We're also 
    // going to test with some primitive types. 
    let const_declarations = vec!
    {
        // Here are type definitions for our enums and structs 
        // above. Attributes from build.rs will not be preserved, 
        // so we need to pass any we want in.
        const_definition!(#[derive(Debug)] TestStruct),
        const_definition!(#[derive(Debug)] TestTup),
        const_definition!(#[derive(Debug)] TestEnum),

        // And here are constant definitions for particular 
        // values.
        const_declaration!(TEST_U8 = 27u8),
        const_declaration!(TEST_F32 = 33.5f32),
        const_declaration!(TEST_VEC = test_vec),
        const_declaration!(TEST_STRING = "I'm a string!"),
        const_declaration!(TEST_COW = 
            std::borrow::Cow::from("Cow!")),
        const_declaration!(TEST_STRUCT = test_struct),
        const_declaration!(TEST_TUP_STRUCT = test_tup_struct),
        const_declaration!(TEST_ENUM = test_enum),
        const_declaration!(TEST_ENUM_TUP = test_enum_tup),
        const_declaration!(TEST_ENUM_STRUCTLIKE = 
            test_enum_structlike)
    }.join("\n");

    // Note: The `const_definition!` and `const_declaration!` 
    // macros above are just simple wrappers for CompileConst 
    // trait methods of the same name. Using those methods
    // would entail the following sytax:
    // TestStruct::const_definition("#[derive(Debug)]")
    // test_struct.const_declaration("TEST_STRUCT")
    // These may be preferable in cases where const names
    // or type attributes have been procedurally generated
    // somehow and need to be treated as strings.

    // If the "phf" feature is enabled, this crate will also 
    // support converting HashMap and HashSet types into 
    // compile-time constant phf map and set types respectively.

    // Lastly, output to the destination file.
    fs::write(&dest_path, const_declarations).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

```

Now, in our `main.rs` file we can do something like this:

```rust

// Include our constants
include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

// And that's it, we can access all of the const values below.
// It plays quite well with rust-analyzer, etc
fn main() 
{
    println!("{}", TEST_U8);
    println!("{}", TEST_F32);
    println!("{:?}", TEST_VEC);
    println!("{}", TEST_STRING);
    println!("{}", TEST_COW);
    println!("{:?}", TEST_STRUCT);
    println!("{:?}", TEST_TUP_STRUCT);
    println!("{:?}", TEST_ENUM);
    println!("{:?}", TEST_ENUM_TUP);
    println!("{:?}", TEST_ENUM_STRUCTLIKE);
}
```

The actual generated output looks like (an unformatted version of) this:
```rust
#[derive(Debug)]
struct TestStruct 
{
    test_u8: u8,
    test_vec: &'static [&'static str],
}
#[derive(Debug)]
struct TestTup(u8, u16);
#[derive(Debug)]
enum TestEnum 
{
    Variant1,
    Variant2(u8),
    Variant3 { named: u8 },
}
const TEST_U8: u8 = 27u8;
const TEST_F32: f32 = 33.5f32;
const TEST_VEC: &'static [u8] = 
    &[1u8, 2u8, 3u8, 4u8, 5u8, 10u8, 4u8];
const TEST_STRING: &'static str = "I'm a string!";
const TEST_COW: &'static str = "Cow!";
const TEST_STRUCT: TestStruct = TestStruct 
{
    test_u8: 12u8,
    test_vec: &["Hello there."],
};
const TEST_TUP_STRUCT: TestTup = TestTup(4u8, 55u16);
const TEST_ENUM: TestEnum = TestEnum::Variant1;
const TEST_ENUM_TUP: TestEnum = TestEnum::Variant2(23u8);
const TEST_ENUM_STRUCTLIKE: TestEnum = TestEnum::Variant3
{ 
    named: 78u8
};
```

## Out-of-the-box Implementations

The following table shows what types have implementations of the CompileConst trait already defined

|Type|Const Representation|
--- | --- 
|\<all numeric primitives\>|no conversion|
|String, &str, str|&'static str|
|Vec\<T\>, &[T]|&'static [T]|
|[T; N where N is 0-99]|[T's CompileConst representation; N]|
|Box\<T\>, Cow\<T\>, Rc\<T\>, Arc\<T\>|T's CompileConst representation|
|HashMap<K,V>|phf::Map\<K, V\>, with K and V's CompileConst representation|
|HashSet\<E\>|phf::Set\<E\>, with E's CompileConst representation|
|()|no conversion|
|\<tuples with 2-16 variants\>|A tuple with the CompileConst representation of each variant|

## Limitations

This crate will use the endianness, pointer widths, etc of the host machine rather than the target, unlike normal consts. It's still unclear if this caveat could be resolved with a reasonable amount of effort.

## Features

At the current time, all features are default. 

### phf
The `phf` feature implements the CompileConst trait for HashMaps and HashSets. It will generate a `phf::Map` for HashMap types and a `phf::Set` for HashSet types. Note that `phf` does NOT need to be included in your build dependencies, but it ought to be included in your runtime dependencies in order to use the constants.

### derive
The `derive` feature adds `#[derive(CompileConst)]` for structs and enums. The requirement is that all members implement `CompileConst` as well.
