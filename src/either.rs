//! Implementation on `Either`

use either::Either;

use crate::CompileConst;

impl<L, R> CompileConst for Either<L, R>
where
    L: CompileConst,
    R: CompileConst,
{
    fn const_type() -> String {
        format!("::either::Either<{},{}>", L::const_type(), R::const_type())
    }

    fn const_val(&self) -> String {
        match self {
            Either::Left(val) => format!("::either::Either::Left({})", val.const_val()),
            Either::Right(val) => format!("::either::Either::Right({})", val.const_val()),
        }
    }
}
