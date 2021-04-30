use proc_macro2::TokenStream;
use quote::quote;

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
    /// Get a string representation of a type. This must be implemented for each
    /// type. Types with generics may need to be able to access an instance of 
    /// one of its generic members and call this function in order to properly
    /// represent the type. Note that this is not necessarily a representation 
    /// of the ACTUAL type, but rather the type that should be used if this data
    /// is going to be represented as a compile-time constant.
    fn const_type() -> TokenStream;
    /// Get a string representation of the current value in constant form. This 
    /// method is where the real magic happens, but self.const_declaration() is
    /// likely the only one you need to call.
    fn const_val(&self) -> TokenStream;
    /// Takes a string (a SCREAMING_SNAKE_CASE string is preferred) to use as a
    /// constant name, then calls self.const_type() and self.const_val() in order
    /// to generate a Rust compile-time constant declaration statement.
    fn const_declaration(&self, name: &str) -> TokenStream 
    {
        let const_type = Self::const_type();
        let const_val = self.const_val();
        quote!
        {
            const #name: #const_type = #const_val
        }
    }
}

/// Helps determine how to represent a type as a string. Simple types ought to
/// just use Constant with an exact string representation, but more complex
/// types (eg, anything with type parameters) will need to set this to Dependant.
#[derive(Debug)]
pub enum ConstType
{
    Constant(TokenStream),
    Dependant
}

impl ConstType
{
    pub fn unwrap(&self) -> &TokenStream
    {
        match self
        {
            Self::Constant(s) => s,
            Self::Dependant => panic!("Unable to determine const type")
        }
    }
}

impl quote::ToTokens for ConstType
{
    fn to_tokens(&self, tokens: &mut TokenStream)
    {
        match self
        {
            Self::Constant(s) => tokens.extend(quote!(#s)),
            Self::Dependant => ()
        }
    }
}

macro_rules! numerics
{
    ( $($t:ty),* ) => 
    {
        $(impl CompileConst for $t
        {
            fn const_type() -> TokenStream 
            { 
                quote!($t)
            }

            fn const_val(&self) -> TokenStream 
            {
                quote!(#self, stringify!($t))
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
            fn const_type() -> TokenStream 
            { 
                quote!(&'static str)
            }
        
            fn const_val(&self) -> TokenStream 
            {
                quote!(#self)
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
            fn const_type() -> TokenStream 
            { 
                quote!(&'static [#(T::const_type())])
            }
        
            fn const_val(&self) -> TokenStream 
            {
                quote!(&[#(self.into_iter().map(|e| e.const_val()).collect::<Vec<String>>().join(","))])
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
            fn const_type() -> TokenStream 
            { 
                T::const_type()
            }
            fn const_val(&self) -> TokenStream 
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
    fn const_type() -> TokenStream 
    {
        quote!(phf::Map<#(K::const_type()), #(V::const_type())>)
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!(phf::phf_map!{ #(self.into_iter().map(|(k,v)| format!("{} => {}", k.const_val(), v.const_val())).collect::<Vec<String>>().join(",")) })
    }
}

#[cfg(feature = "phf")]
impl<E: CompileConst> CompileConst for HashSet<E>
{
    fn const_type() -> TokenStream 
    {
        quote!(phf::Set<#(E::const_type())>)
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!(phf::phf_set!{#(self.into_iter().map(|e| format!("{}", e.const_val())).collect::<Vec<String>>().join(","))})
    }
}

// Implementation for various-sized tuples

impl CompileConst for ()
{
    fn const_type() -> TokenStream 
    {
        quote!(())
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!(#(Self::CONST_TYPE))
    }
}

impl<T: CompileConst, U: CompileConst> CompileConst for (T, U)
{
    fn const_type() -> TokenStream 
    {
        quote!((#(T::const_type()),#(U::const_type())))
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!((#(self.0.const_val()),#(self.1.const_val())))
    }
}

impl<T, U, V> CompileConst for (T, U, V)
    where T: CompileConst, U: CompileConst, V: CompileConst
{
    fn const_type() -> TokenStream 
    {
        quote!((#(T::const_type()),#(U::const_type()),#(V::const_type())))
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!((#(self.0.const_val()),#(self.1.const_val()),#(self.2.const_val())))
    }
}

impl<T, U, V, W> CompileConst for (T, U, V, W)
    where T: CompileConst, U: CompileConst, V: CompileConst,
          W: CompileConst
{
    fn const_type() -> TokenStream 
    {
        quote!((#(T::const_type()),#(U::const_type()),#(V::const_type()),#(W::const_type())))
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!((#(self.0.const_val()),#(self.1.const_val()),#(self.2.const_val()),#(self.3.const_val())))
    }
}

impl<T, U, V, W, X> CompileConst for (T, U, V, W, X)
    where T: CompileConst, U: CompileConst, V: CompileConst,
          W: CompileConst, X: CompileConst
{
    fn const_type() -> TokenStream 
    {
        quote!
        (
            (#(T::const_type()),
            #(U::const_type()),
            #(V::const_type()),
            #(W::const_type()),
            #(X::const_type()))
        )
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!
        (
            (#(self.0.const_val()),
            #(self.1.const_val()),
            #(self.2.const_val()),
            #(self.3.const_val()),
            #(self.4.const_val()))
        )
    }
}

impl<T, U, V, W, X, Y> CompileConst for (T, U, V, W, X, Y)
    where T: CompileConst, U: CompileConst, V: CompileConst,
          W: CompileConst, X: CompileConst, Y: CompileConst
{
    fn const_type() -> TokenStream 
    {
        quote!
        (
            (#(T::const_type()),
            #(U::const_type()),
            #(V::const_type()),
            #(W::const_type()),
            #(X::const_type()),
            #(Y::const_type()))
        )
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!
        (
            (#(self.0.const_val()),
            #(self.1.const_val()),
            #(self.2.const_val()),
            #(self.3.const_val()),
            #(self.4.const_val()),
            #(self.5.const_val()))
        )
    }
}

impl<T, U, V, W, X, Y, Z> CompileConst for (T, U, V, W, X, Y, Z)
    where T: CompileConst, U: CompileConst, V: CompileConst,
          W: CompileConst, X: CompileConst, Y: CompileConst, 
          Z: CompileConst
{
    fn const_type() -> TokenStream 
    {
        quote!
        (
            (#(T::const_type()),
            #(U::const_type()),
            #(V::const_type()),
            #(W::const_type()),
            #(X::const_type()),
            #(Y::const_type()),
            #(Z::const_type()))
        )
    }

    fn const_val(&self) -> TokenStream 
    {
        quote!
        (
            (#(self.0.const_val()),
            #(self.1.const_val()),
            #(self.2.const_val()),
            #(self.3.const_val()),
            #(self.4.const_val()),
            #(self.5.const_val()),
            #(self.6.const_val()))
        )
    }
}