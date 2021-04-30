#[cfg(feature = "phf")]
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(feature = "derive")]
pub use const_gen_derive::*;

#[cfg(test)]
mod test;
/// Trait which defines how a type should be represented as a constant
pub trait CompileConst
{
    const CONST_TYPE: ConstType;

    /// Get a string representation of a type. This must be implemented for each
    /// type. Types with generics may need to be able to access an instance of 
    /// one of its generic members and call this function in order to properly
    /// represent the type. Note that this is not necessarily a representation 
    /// of the ACTUAL type, but rather the type that should be used if this data
    /// is going to be represented as a compile-time constant.
    fn const_type(&self) -> String;
    /// Get a string representation of the current value in constant form. This 
    /// method is where the real magic happens, but self.const_declaration() is
    /// likely the only one you need to call.
    fn const_val(&self) -> String;
    /// Takes a string (a SCREAMING_SNAKE_CASE string is preferred) to use as a
    /// constant name, then calls self.const_type() and self.const_val() in order
    /// to generate a Rust compile-time constant declaration statement.
    fn const_declaration(&self, name: &str) -> String 
    {
        format!("const {}: {} = {};", name, self.const_type(), self.const_val())
    }
}

/// Helps determine how to represent a type as a string. Simple types ought to
/// just use Constant with an exact string representation, but more complex
/// types (eg, anything with type parameters) will need to set this to Dependant.
#[derive(Debug)]
pub enum ConstType
{
    Constant(&'static str),
    Dependant
}

impl ConstType
{
    pub fn unwrap(&self) -> &'static str
    {
        match self
        {
            Self::Constant(s) => s,
            Self::Dependant => panic!("Unable to determine const type")
        }
    }
}

macro_rules! numerics
{
    ( $($t:ty),* ) => 
    {
        $(impl CompileConst for $t
        {
            const CONST_TYPE: ConstType = ConstType::Constant(stringify!($t)); 
            fn const_type(&self) -> String 
            { 
                Self::CONST_TYPE.unwrap().to_string()
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
        $(impl CompileConst for $t
        {
            const CONST_TYPE: ConstType = ConstType::Constant("&'static str");
            fn const_type(&self) -> String 
            { 
                Self::CONST_TYPE.unwrap().to_string()
            }
        
            fn const_val(&self) -> String 
            {
                format!("\"{}\"", self)
            }
        })*
    }
}
strings!(String, &str, str);

macro_rules! arrays
{
    ( $($t:ty),* ) => 
    {
        $(impl<T: CompileConst> CompileConst for $t
        {
            const CONST_TYPE: ConstType = ConstType::Dependant;
            fn const_type(&self) -> String 
            { 
                match self.iter().next()
                {
                    Some(t) => format!("&'static [{}]", t.const_type()),
                    None => format!("&'static [{}]", T::CONST_TYPE.unwrap())
                }
            }
        
            fn const_val(&self) -> String 
            {
                format!("&[{}]", self
                    .into_iter()
                    .map(|e| e.const_val())
                    .collect::<Vec<String>>()
                    .join(","))
            }
        })*
    }
}
arrays!(Vec<T>, &[T]);

macro_rules! derefs
{
    ( $($t:ty $(=> $bound:tt)?),* ) => 
    {
        $(impl<T: CompileConst $(+ $bound)? > CompileConst for $t
        {
            const CONST_TYPE: ConstType = T::CONST_TYPE;
            fn const_type(&self) -> String 
            { 
                (**self).const_type()
            }
            fn const_val(&self) -> String 
            {
                (**self).const_val()
            }
        })*
    }
}
derefs!
(
    Box<T>,
    std::borrow::Cow<'_, T> => Clone, 
    std::rc::Rc<T>, 
    std::sync::Arc<T>
);

#[cfg(feature = "phf")]
impl<K: CompileConst, V: CompileConst> CompileConst for HashMap<K,V>
{
    const CONST_TYPE: ConstType = ConstType::Dependant;
    fn const_type(&self) -> String 
    {
        match self.iter().next()
        {
            Some((k,v)) => format!("phf::Map<{}, {}>", k.const_type(), v.const_type()),
            None => format!("phf::Map<{}, {}>", K::CONST_TYPE.unwrap(), V::CONST_TYPE.unwrap())
        }
    }

    fn const_val(&self) -> String 
    {
        format!("phf::phf_map!{{{}}}", self
            .into_iter()
            .map(|(k,v)| format!("{} => {}", k.const_val(), v.const_val()))
            .collect::<Vec<String>>()
            .join(","))
    }
}

#[cfg(feature = "phf")]
impl<E: CompileConst> CompileConst for HashSet<E>
{
    const CONST_TYPE: ConstType = ConstType::Dependant;
    fn const_type(&self) -> String 
    {
        match self.iter().next()
        {
            Some(e) => format!("phf::Set<{}>", e.const_type()),
            None => format!("phf::Set<{}>", E::CONST_TYPE.unwrap())
        }
    }

    fn const_val(&self) -> String 
    {
        format!("phf::phf_set!{{{}}}", self
            .into_iter()
            .map(|e| format!("{}", e.const_val()))
            .collect::<Vec<String>>()
            .join(","))
    }
}