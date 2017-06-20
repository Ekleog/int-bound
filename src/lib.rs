// TODO: use `num` crate in order to wrap any `Num` type

#[macro_use] extern crate typenum;

use std::marker::PhantomData;
use std::ops::{Add, BitAnd, Deref};
use typenum as tn;

pub trait Constraint {
    fn check(n: isize) -> bool;
}

pub struct Range<Low: tn::Integer, High: tn::Integer>(PhantomData<(Low, High)>);

impl<Low: tn::Integer, High: tn::Integer> Constraint for Range<Low, High> {
    fn check(n: isize) -> bool {
        Low::to_isize() <= n && n <= High::to_isize()
    }
}

pub trait Integer<C: Constraint>: Deref<Target=isize> {}

pub struct Int<C: Constraint> {
    val: isize,
    _constraint: PhantomData<C>,
}

impl<C: Constraint> Deref for Int<C> {
    type Target = isize;

    fn deref(&self) -> &isize {
        &self.val
    }
}

impl<L1: tn::Integer + tn::Cmp<L2>, H1: tn::Integer + tn::Cmp<H2>, L2: tn::Integer, H2: tn::Integer>
    Integer<Range<L2, H2>> for Int<Range<L1, H1>>
    where op!((L1 >= L2) & (H1 <= H2)): tn::NonZero,
          // TODO: there *must* be a better way than just following rustc's instructions
          H1: tn::private::IsLessOrEqualPrivate<H2, <H1 as tn::Cmp<H2>>::Output>,
          L1: tn::private::IsGreaterOrEqualPrivate<L2, <L1 as tn::Cmp<L2>>::Output>,
          <L1 as tn::private::IsGreaterOrEqualPrivate<L2, <L1 as tn::Cmp<L2>>::Output>>::Output: BitAnd<<H1 as tn::private::IsLessOrEqualPrivate<H2, <H1 as tn::Cmp<H2>>::Output>>::Output>
{ }

impl<L1: tn::Integer + Add<L2>, H1: tn::Integer + Add<H2>, L2: tn::Integer, H2: tn::Integer>
        Add<Int<Range<L1, H1>>> for Int<Range<L2, H2>>
        where op!(L1 + L2): tn::Integer, op!(H1 + H2): tn::Integer {
    type Output = Int<Range<op!(L1 + L2), op!(H1 + H2)>>;

    fn add(self, rhs: Int<Range<L1, H1>>) -> Self::Output {
        Int {
            val: self.val + rhs.val,
            _constraint: PhantomData,
        }
    }
}

pub fn constant<V: tn::Integer>() -> Int<Range<V, V>> {
    Int {
        val: V::to_isize(),
        _constraint: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_42<I: Integer<Range<tn::P41, tn::P43>>>(x: I) {
        assert_eq!(*x, 42);
    }

    #[test]
    fn it_works() {
        assert_eq!(*constant::<tn::N5>(), -5);
        assert_eq!(*(constant::<tn::N5>() + constant::<tn::P7>()), 2);
        check_42(constant::<tn::P24>() + constant::<tn::P18>());
    }
}
