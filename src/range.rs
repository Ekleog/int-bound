use std::marker::PhantomData;
use std::ops::{Add, BitAnd};
use typenum as tn;
use typenum::Cmp;

pub trait RangeTrait {
    fn check(n: isize) -> bool;
}

pub struct Range<Low: tn::Integer, High: tn::Integer>(PhantomData<(Low, High)>);

impl<Low: tn::Integer, High: tn::Integer> RangeTrait for Range<Low, High> {
    fn check(n: isize) -> bool {
        Low::to_isize() <= n && n <= High::to_isize()
    }
}

impl<L1: tn::Integer, H1: tn::Integer, L2: tn::Integer, H2: tn::Integer>
        Add<Range<L1, H1>> for Range<L2, H2>
        where L1: Add<L2>,
              H1: Add<H2>,
              op!(L1 + L2): tn::Integer,
              op!(H1 + H2): tn::Integer {
    type Output = Range<op!(L1 + L2), op!(H1 + H2)>;

    fn add(self, _: Range<L1, H1>) -> Self::Output {
        Range(PhantomData)
    }
}

pub trait SubRange<T: RangeTrait> {
    type Output;
}

impl<L1: tn::Integer, H1: tn::Integer, L2: tn::Integer, H2: tn::Integer>
        SubRange<Range<L1, H1>> for Range<L2, H2>
        where L1: Cmp<L2>,
              H2: Cmp<H1>,
              // TODO: there must be a way not involving typenum::private
              L1: tn::private::IsLessOrEqualPrivate<L2, <L1 as tn::Cmp<L2>>::Output>,
              H2: tn::private::IsLessOrEqualPrivate<H1, <H2 as tn::Cmp<H1>>::Output>,
              op!(L1 <= L2): BitAnd<op!(H2 <= H1)> {
    type Output = op!((L1 <= L2) & (H2 <= H1));
}
