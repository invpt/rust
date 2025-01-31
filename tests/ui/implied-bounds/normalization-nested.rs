// Test for normalization of projections that appear in the item bounds
// (versus those that appear directly in the input types).
//
// revisions: param_ty lifetime
// check-pass

pub trait Iter {
    type Item;
}

#[cfg(param_ty)]
impl<X, I> Iter for I
where
    I: IntoIterator<Item = X>,
{
    type Item = X;
}

#[cfg(lifetime)]
impl<'x, I> Iter for I
where
    I: IntoIterator<Item = &'x ()>,
{
    type Item = &'x ();
}

pub struct Map<I>(I)
where
    I: Iter,
    I::Item: 'static;

pub fn test_wfcheck<'x>(_: Map<Vec<&'x ()>>) {}

pub fn test_borrowck<'x>(_: Map<Vec<&'x ()>>, s: &'x str) -> &'static str {
    s
}

fn main() {}
