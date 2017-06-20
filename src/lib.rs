// TODO: use `num` crate in order to wrap any `Num` type

#[macro_use] extern crate typenum;

use std::marker::PhantomData;
use std::ops::{Add, Deref};
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

    fn check_42(x: Int<Range<tn::P42, tn::P42>>) {
        assert_eq!(*x, 42);
    }

    #[test]
    fn it_works() {
        assert_eq!(*constant::<tn::N5>(), -5);
        assert_eq!(*(constant::<tn::N5>() + constant::<tn::P7>()), 2);
        check_42(constant::<tn::P24>() + constant::<tn::P18>());
    }
}
