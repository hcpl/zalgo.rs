use core::iter;
#[cfg(feature = "nightly")]
use core::ops;

use is_zalgo;



/// Returns an [`Iterator`] of non-Zalgo `char`s that remained from the original
/// iterator.
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
pub fn unapply_iter<I: Iterator<Item = char>>(chars: I) -> UnapplyIter<I> {
    UnapplyIter {
        inner: chars.filter(|c| !is_zalgo(*c)),
    }
}

/// An iterator of non-Zalgo `char`s that remained from the original iterator.
///
/// This struct is created by the [`unapply_iter`] function. See its
/// documentation for more details.
///
/// [`unapply_iter`]: fn.unapply_iter.html
pub struct UnapplyIter<I> {
    inner: iter::Filter<I, fn(&char) -> bool>,
}

// Simply delegate methods to `inner` which likely have good implementation
// already thanks to `Filter` being in standard library.
impl<I> Iterator for UnapplyIter<I>
    where I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn last(self) -> Option<char> {
        self.inner.last()
    }

    fn nth(&mut self, n: usize) -> Option<char> {
        self.inner.nth(n)
    }

    #[cfg(feature = "nightly")]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
        where Self: Sized,
              Fold: FnMut(Acc, char) -> R,
              R: ops::Try<Ok = Acc>,
    {
        self.inner.try_fold(init, fold)
    }

    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, char) -> Acc,
    {
        self.inner.fold(init, fold)
    }
}

impl<I> DoubleEndedIterator for UnapplyIter<I>
    where I: Iterator<Item = char> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<char> {
        self.inner.next_back()
    }

    #[cfg(feature = "nightly")]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
        where Self: Sized,
              Fold: FnMut(Acc, char) -> R,
              R: ops::Try<Ok = Acc>,
    {
        self.inner.try_rfold(init, fold)
    }

    #[cfg(any(feature = "nightly", stable_iter_rfold))]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, char) -> Acc,
    {
        self.inner.rfold(init, fold)
    }
}

#[cfg(any(feature = "nightly", stable_fused_iterator))]
impl<I> iter::FusedIterator for UnapplyIter<I>
    where I: Iterator<Item = char> + iter::FusedIterator,
{}