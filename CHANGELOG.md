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