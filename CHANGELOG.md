# 1.6.0
- Now supports inheriting documentation of enum variants.

# 1.5.0
- Added `static_declaration` to generate `static` constants instead of `const` constants.

# 1.4.0 
- Added no_std support, enabled by disabled the `std` default feature.

# 1.3.0
- Added #[inherit_docs] attribute that can be added to anything with "#[derive(CompileConst)]"
- Added the ability to generate attributes for declarations, meaning it's now possible to add rustdocs to generated items.

# 1.2.0
- Fixed a bug which prevented struct fields from inheriting visibility modifiers.

# 1.1.0
- Implemented CompileConst for bool.

# 1.0
- No significant changes, but enough testing has been done to prove this works. Going 1.0 to signify that.

# 0.4
- Added support for visability modifiers.
- Removed a useless macro.

# 0.3
- Added const array generation with a trait.

# 0.2.5
- Added `const_definition!` and `const_declaration!` macros to simplify creation of constants.

# 0.2
- It's now possible generate definitions of structs, enums, and unions defined in build.rs. See the readme for details.
- Removed the CONST_TYPE associated constant from the trait. const_type() no longer takes a `self` param, simplifying
the API quite a bit.

# 0.1
- Initial release