# const-gen

This is a crate for generating compile-time constants in your `build.rs` file. This crate supports converting types that are typically heap-allocated into fixed-size constants. It includes support for primitives, strings, vectors, maps, sets, and comes with a derive macro to allow implementation with structs and enums.

See this example:
```rust
// build.rs

// First, let's dummy up some structs. Enabling the "derive" feature allows
// us to do this simply, but implementing the CompileConst trait 

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
    // Use the OUT_DIR environment variable to get an appropriate path.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");

    // Now let's dummy up some data to use in our const generation
    let test_vec: Vec<u8> = vec!(1,2,3,4,5,10,4);
    let test_struct = TestStruct{ test_u8: 12, test_vec: vec!(String::from("Hello there.")) };
    let test_tup_struct = TestTup(4, 55,);
    let test_enum = TestEnum::Variant1;
    let test_enum_tup = TestEnum::Variant2(23);
    let test_enum_structlike = TestEnum::Variant3{ named: 78 };

    // Now we'll generate the const declarations. We're also going to test with some
    // primitive types. 
    let const_declarations = vec!
    {
        27u8.const_declaration("TEST_U8"),
        33.5f32.const_declaration("TEST_F32"),
        test_vec.const_declaration("TEST_VEC"),
        "I'm a string!".const_declaration("TEST_STRING"),
        std::borrow::Cow::from("Cow!").const_declaration("TEST_COW"),
        test_struct.const_declaration("TEST_STRUCT"),
        test_tup_struct.const_declaration("TEST_TUP_STRUCT"),
        test_enum.const_declaration("TEST_ENUM"),
        test_enum_tup.const_declaration("TEST_ENUM_TUP"),
        test_enum_structlike.const_declaration("TEST_ENUM_STRUCTLIKE")
    }.join("\n");

    // If the "phf" feature is enabled, this crate will also support converting
    // HashMap and HashSet types into compile-time constant phf map and set types 
    // respectively.

    // Lastly, output to the destination file.
    fs::write(&dest_path, const_declarations).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

```

Now, in our `main.rs` file we can do something like this:

```rs

// Include our constants
include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

// This struct mirrors the TestStruct we defined in build.rs, but the 
// heap-allocated vectors and strings have been replaced with static slices
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
    Variant3{ named: u8 }
}

// And that's it, we can access all of the const values below. It plays quite
// well with rust-analyzer, etc
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