#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![forbid(clippy::format_push_string)]

use core::net::*;

#[cfg(not(feature = "std"))]
include!("no_std.rs");
#[cfg(feature = "std")]
use std::{borrow::Cow, collections::HashSet, fmt::Display, rc::Rc, sync::Arc};

#[cfg(feature = "phf")]
use std::collections::HashMap;

#[cfg(feature = "derive")]
pub use const_gen_derive::*;

#[cfg(test)]
mod test;

#[cfg(feature = "either")]
mod either;

/// A macro to help in the creation of const definitions. Allows this syntax:
/// `const_definition!(#[attribute1] #[attributeN] visibility TypeName)`
/// This is syntactic sugar for calling the `CompileConst::const_definition`
/// function.
#[macro_export]
macro_rules! const_definition
{
    ( $(#[$attr:meta])* $vis:vis $ty:ty) =>
    {
        <$ty>::const_definition(stringify!($(#[$attr])*), stringify!($vis))
    }
}

/// A macro to help in the creation of const declarations. Allows this syntax:
/// `const_declaration!(visibility VAR_NAME = value)`
/// This is syntactic sugar for calling the `CompileConst::const_declaration`
/// function.
#[macro_export]
macro_rules! const_declaration
{
    ( $(#[$attr:meta])* $vis:vis $name:ident = $($val:tt)*) =>
    {
        $($val)*.const_declaration(stringify!($(#[$attr])*), stringify!($vis), stringify!($name))
    }
}

/// A macro to help in the creation of static declarations. Allows this syntax:
/// `static_declaration!(visibility VAR_NAME = value)`
/// This is syntactic sugar for calling the `CompileConst::static_declaration`
/// function.
#[macro_export]
macro_rules! static_declaration
{
    ( $(#[$attr:meta])* $vis:vis $name:ident = $($val:tt)*) =>
    {
        $($val)*.static_declaration(stringify!($(#[$attr])*), stringify!($vis), stringify!($name))
    }
}

/// Like const_declaration, but for const array types
#[macro_export]
macro_rules! const_array_declaration
{
    ( $(#[$attr:meta])* $vis:vis $name:ident = $($val:tt)*) =>
    {
        $($val)*.const_array_declaration(stringify!($(#[$attr])*), stringify!($vis), stringify!($name))
    }
}

/// Like static_declaration, but for const array types
#[macro_export]
macro_rules! static_array_declaration
{
    ( $(#[$attr:meta])* $vis:vis $name:ident = $($val:tt)*) =>
    {
        $($val)*.static_array_declaration(stringify!($(#[$attr])*), stringify!($vis), stringify!($name))
    }
}

/// Enum representing the type of declaration to generate, e.g. `const` or `static`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeclarationType {
    Const,
    Static,
}

impl Display for DeclarationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DeclarationType::Const => "const",
            DeclarationType::Static => "static",
        })
    }
}

/// Trait which defines how a type should be represented as a constant
pub trait CompileConst {
    /// Get a string representation of a type. This must be implemented for each
    /// type. Note that this is not necessarily a representation
    /// of the ACTUAL type, but rather the type that should be used if this data
    /// is going to be represented as a compile-time constant.
    fn const_type() -> String;
    /// Get a string representation of the current value in constant form.
    fn const_val(&self) -> String;
    /// Generates the declaration statement string.
    ///
    /// Takes 3 strings: Attributes, a visibility (eg pub) and a name (a SCREAMING_SNAKE_CASE string is preferred),
    /// plus a [declaration type](DeclarationType) (e.g. const or static).
    ///
    /// It then constructs a valid Rust declaration statement using the type and value of the current object by calling [const_val()](CompileConst::const_val) and [const_type()](CompileConst::const_type).
    ///
    ///```rust
    /// use const_gen::{CompileConst, DeclarationType};
    ///
    /// let test_str_declaration = "I'm a string!".declaration(
    ///    "#[allow(dead_code)]",
    ///    "pub(crate)",
    ///    DeclarationType::Const,
    ///    "TEST_STR",
    /// );
    /// assert_eq!(
    ///    test_str_declaration,
    ///    r#"#[allow(dead_code)] pub(crate) const TEST_STR: &str = "I'm a string!";"#
    /// );
    ///```
    fn declaration(
        &self,
        attrs: &str,
        vis: &str,
        declaration_type: DeclarationType,
        name: &str,
    ) -> String {
        format!(
            "{}{}{}{}{} {}: {} = {};",
            if attrs.is_empty() { "" } else { attrs },
            if attrs.is_empty() { "" } else { " " },
            vis,
            if vis.is_empty() { "" } else { " " },
            declaration_type,
            name,
            Self::const_type(),
            self.const_val()
        )
    }
    /// Generates the declaration statement string for a `const` declaration.
    ///
    /// See [declaration()](CompileConst::declaration) for more information.
    fn const_declaration(&self, attrs: &str, vis: &str, name: &str) -> String {
        self.declaration(attrs, vis, DeclarationType::Const, name)
    }
    /// Generates the declaration statement string for a `static` declaration.
    ///
    /// See [declaration()](CompileConst::declaration) for more information.
    fn static_declaration(&self, attrs: &str, vis: &str, name: &str) -> String {
        self.declaration(attrs, vis, DeclarationType::Static, name)
    }
    /// Return a const definition for this type. Attributes may be included, and
    /// must be formatted as the compiler would expect to see them (including
    /// the pound sign and square brackets `"#[...]"`). Always returns an empty
    /// string for types defined in the standard library. Typically this is
    /// easier to call instead through the const_definition! macro. Visibility
    /// modifiers (eg, pub(...)) may be used, or an empty string passed in to
    /// generate a private item.
    fn const_definition(_attrs: &str, _vis: &str) -> String {
        String::new()
    }
}

/// Trait which defines how an array-representable type should be represented as a const array
pub trait CompileConstArray {
    /// Like [const_type](CompileConst::const_type), but for a fixed-size array.
    fn const_array_type(&self) -> String;
    /// Like [const_val](CompileConst::const_val), but for a fixed-size array.
    fn const_array_val(&self) -> String;
    /// Like [declaration](CompileConst::declaration), but for a fixed-size array.
    fn array_declaration(
        &self,
        attrs: &str,
        vis: &str,
        declaration_type: DeclarationType,
        name: &str,
    ) -> String {
        format!(
            "{}{}{}{}{} {}: {} = {};",
            if attrs.is_empty() { "" } else { attrs },
            if attrs.is_empty() { "" } else { " " },
            vis,
            if vis.is_empty() { "" } else { " " },
            declaration_type,
            name,
            self.const_array_type(),
            self.const_array_val()
        )
    }

    /// Like [const_declaration](CompileConst::const_declaration), but for a fixed-size array.
    fn const_array_declaration(&self, attrs: &str, vis: &str, name: &str) -> String {
        self.array_declaration(attrs, vis, DeclarationType::Const, name)
    }
    /// Like [static_declaration](CompileConst::static_declaration), but for a fixed-size array.
    fn static_array_declaration(&self, attrs: &str, vis: &str, name: &str) -> String {
        self.array_declaration(attrs, vis, DeclarationType::Static, name)
    }
}

macro_rules! numerics
{
    ( $($t:ty),* ) =>
    {
        $(impl CompileConst for $t
        {
            fn const_type() -> String
            {
                stringify!($t).to_string()
            }

            fn const_val(&self) -> String
            {
                format!("{}{}", self, stringify!($t))
            }
        })*
    }
}
numerics!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! strings
{
    ( $($t:ty),* ) =>
    {
        $(
        impl CompileConst for $t
        {
            fn const_type() -> String
            {
                "&'static str".to_string()
            }

            fn const_val(&self) -> String
            {
                format!("\"{}\"", self)
            }
        }
        impl CompileConstArray for $t
        {
            fn const_array_type(&self) -> String
            {
                format!("[char; {}]", self.chars().count())
            }

            fn const_array_val(&self) -> String
            {
                format!("[{}]", self.chars().map(|c| format!("'{}',", c)).collect::<Vec<String>>().concat())
            }
        }
        )*
    }
}
strings!(String, &str, str);

macro_rules! slices
{
    ( $($t:ty),* ) =>
    {
        $(
        impl<T: CompileConst> CompileConst for $t
        {
            fn const_type() -> String
            {
                format!("&'static [{}]", T::const_type())
            }

            fn const_val(&self) -> String
            {
                format!("&[{}]", self
                    .into_iter()
                    .map(|e| e.const_val())
                    .collect::<Vec<String>>()
                    .join(","))
            }
        }
        impl<T: CompileConst> CompileConstArray for $t
        {
            fn const_array_type(&self) -> String
            {
                format!("[{}; {}]", T::const_type(), self.iter().count())
            }

            fn const_array_val(&self) -> String
            {
                format!("[{}]", self
                    .into_iter()
                    .map(|e| e.const_val())
                    .collect::<Vec<String>>()
                    .join(","))
            }
        }
        )*
    }
}
slices!(Vec<T>, &[T]);

macro_rules! derefs
{
    ( $($t:ty $(=> $bound:tt)?),* ) =>
    {
        $(
        impl<T: CompileConst $(+ $bound)? > CompileConst for $t
        {
            fn const_type() -> String
            {
                T::const_type()
            }
            fn const_val(&self) -> String
            {
                (**self).const_val()
            }
        }
        impl<T: CompileConstArray $(+ $bound)? > CompileConstArray for $t
        {
            fn const_array_type(&self) -> String
            {
                (**self).const_array_type()
            }

            fn const_array_val(&self) -> String
            {
                (**self).const_array_val()
            }
        }
        )*
    }
}
derefs!(
    Box<T>,
    Cow<'_, T> => Clone,
    Rc<T>,
    Arc<T>
);

impl CompileConst for char {
    fn const_type() -> String {
        "char".to_owned()
    }

    fn const_val(&self) -> String {
        format!("'{}'", *self)
    }
}

impl CompileConst for bool {
    fn const_type() -> String {
        "bool".to_owned()
    }

    fn const_val(&self) -> String {
        if *self { "true" } else { "false" }.to_owned()
    }
}

impl<T: CompileConst> CompileConst for Option<T> {
    fn const_type() -> String {
        format!("Option<{}>", T::const_type())
    }

    fn const_val(&self) -> String {
        match self {
            Some(t) => format!("Some({})", t.const_val()),
            None => String::from("None"),
        }
    }
}

#[cfg(feature = "phf")]
impl<K: CompileConst, V: CompileConst> CompileConst for HashMap<K, V> {
    fn const_type() -> String {
        format!("phf::Map<{}, {}>", K::const_type(), V::const_type())
    }

    fn const_val(&self) -> String {
        format!(
            "phf::phf_map!{{{}}}",
            self.iter()
                .map(|(k, v)| format!("{} => {}", k.const_val(), v.const_val()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[cfg(feature = "phf")]
impl<E: CompileConst> CompileConst for HashSet<E> {
    fn const_type() -> String {
        format!("phf::Set<{}>", E::const_type())
    }

    fn const_val(&self) -> String {
        format!(
            "phf::phf_set!{{{}}}",
            self.iter()
                .map(|e| e.const_val().to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl CompileConst for Ipv4Addr {
    fn const_type() -> String {
        "core::net::Ipv4Addr".to_owned()
    }

    fn const_val(&self) -> String {
        format!(
            "core::net::Ipv4Addr::new({})",
            self.octets()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl CompileConst for Ipv6Addr {
    fn const_type() -> String {
        "core::net::Ipv6Addr".to_owned()
    }

    fn const_val(&self) -> String {
        format!(
            "core::net::Ipv6Addr::new({})",
            self.segments()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl CompileConst for IpAddr {
    fn const_type() -> String {
        "core::net::IpAddr".to_owned()
    }

    fn const_val(&self) -> String {
        match self {
            IpAddr::V4(ipv4) => format!("core::net::IpAddr::V4({})", ipv4.const_val()),
            IpAddr::V6(ipv6) => format!("core::net::IpAddr::V6({})", ipv6.const_val()),
        }
    }
}

impl CompileConst for SocketAddr {
    fn const_type() -> String {
        "core::net::SocketAddr".to_owned()
    }

    fn const_val(&self) -> String {
        format!(
            "core::net::SocketAddr::new({}, {})",
            self.ip().const_val(),
            self.port()
        )
    }
}

impl CompileConst for SocketAddrV4 {
    fn const_type() -> String {
        "core::net::SocketAddrV4".to_owned()
    }

    fn const_val(&self) -> String {
        format!(
            "core::net::SocketAddrV4::new({}, {})",
            self.ip().const_val(),
            self.port()
        )
    }
}

impl CompileConst for SocketAddrV6 {
    fn const_type() -> String {
        "core::net::SocketAddrV6".to_owned()
    }

    fn const_val(&self) -> String {
        format!(
            "core::net::SocketAddrV6::new({}, {}, {}, {})",
            self.ip().const_val(),
            self.port(),
            self.flowinfo(),
            self.scope_id()
        )
    }
}


macro_rules! arrays
{
    ($($n:literal),*) =>
    {
        $(impl<T: CompileConst> CompileConst for [T; $n]
        {
            fn const_type() -> String
            {
                format!("[{}; {}]", T::const_type(), $n)
            }

            fn const_val(&self) -> String
            {
                format!("[{}]", self
                    .iter()
                    .map(|e| e.const_val())
                    .collect::<Vec<String>>()
                    .join(","))
            }
        })*
    }
}
arrays!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256
);

// Implementation for various-sized tuples
macro_rules! tuples
{
    ($format:literal $(, $ty:ident $index:tt)*) =>
    {
        impl<$($ty: CompileConst),*> CompileConst for ($($ty),*)
        {
            fn const_type() -> String
            {
                format!($format, $($ty::const_type()),*)
            }

            fn const_val(&self) -> String
            {
                format!($format, $(self.$index.const_val()),*)
            }
        }

        impl<$($ty: CompileConstArray),*> CompileConstArray for ($($ty),*)
        {
            fn const_array_type(&self) -> String
            {
                format!($format, $(self.$index.const_array_type()),*)
            }

            fn const_array_val(&self) -> String
            {
                format!($format, $(self.$index.const_array_val()),*)
            }
        }
    }
}

tuples!("()");
tuples!("({},{})", A 0, B 1);
tuples!("({},{},{})", A 0, B 1, C 2);
tuples!("({},{},{},{})", A 0, B 1, C 2, D 3);
tuples!("({},{},{},{},{})", A 0, B 1, C 2, D 3, E 4);
tuples!("({},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5);
tuples!("({},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6);
tuples!("({},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
tuples!("({},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
tuples!("({},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
tuples!("({},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
tuples!("({},{},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
tuples!("({},{},{},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
tuples!("({},{},{},{},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
tuples!("({},{},{},{},{},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
tuples!("({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})", A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
