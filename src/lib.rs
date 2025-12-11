pub use oonum_macros::{dispatch, oonum};

pub trait Sub<S: ?Sized> {
    fn borrow_super(supe: &S) -> Option<&Self>;
    fn borrow_super_mut(supe: &mut S) -> Option<&mut Self>;
    fn from_super(supe: S) -> Option<Self>
    where
        Self: Sized;
    fn into_super(self) -> S;

    fn can_borrow_super(supe: &S) -> bool {
        Self::borrow_super(supe).is_some()
    }
}

pub trait Super {
    fn as_sub<S: Sub<Self>>(&self) -> Option<&S> {
        S::borrow_super(self)
    }
    fn as_sub_mut<S: Sub<Self>>(&mut self) -> Option<&mut S> {
        S::borrow_super_mut(self)
    }
    fn into_sub<S: Sub<Self> + Sized>(self) -> Option<S>
    where
        Self: Sized,
    {
        S::from_super(self)
    }
    fn from_sub<S: Sub<Self>>(sub: S) -> Self
    where
        Self: Sized,
    {
        sub.into_super()
    }

    fn is_sub<S: Sub<Self>>(&self) -> bool {
        S::can_borrow_super(self)
    }
}

impl<T> Super for T {}

pub trait Discriminant<O> {
    const DISCRIMINANT: u16;
}
