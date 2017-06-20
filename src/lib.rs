// TODO: use `num` crate in order to wrap any `Num` type

#[macro_use] extern crate typenum;

use std::marker::PhantomData;
use std::ops::{Add, Deref};
use typenum as tn;

mod range;
use range::*;

pub trait Integer<C: RangeTrait>: Deref<Target=isize> {}

pub struct Int<C: RangeTrait> {
    val: isize,
    _constraint: PhantomData<C>,
}

impl<C: RangeTrait> Deref for Int<C> {
    type Target = isize;

    fn deref(&self) -> &isize {
        &self.val
    }
}

impl<R1: RangeTrait, R2: RangeTrait> Integer<R1> for Int<R2>
    where R2: SubRange<R1>,
          <R2 as SubRange<R1>>::Output: tn::NonZero
{ }

impl<R1: RangeTrait, R2: RangeTrait> Add<Int<R1>> for Int<R2>
        where R1: Add<R2>,
              op!(R1 + R2): RangeTrait {
    type Output = Int<op!(R1 + R2)>;

    fn add(self, rhs: Int<R1>) -> Self::Output {
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
