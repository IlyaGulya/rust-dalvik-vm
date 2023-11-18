use std::rc::Rc;

use crate::gen::smali::smaliparser::SmaliParserContext;

pub trait TryDowncast: Sized {
    type Error;
    fn try_downcast_rc<'a>(ctx: Rc<dyn SmaliParserContext<'a>>) -> Result<Rc<Self>, Self::Error>;
}

// pub trait TryDowncastInto<'a, T>: Sized {
//     type Error;
//     fn try_downcast_into(self: Rc<Self>) -> Result<T, Self::Error>;
// }

// // TODO
// impl<'a, T, U> TryDowncastInto<'a, Rc<U>> for T
//     where
//         T: 'a + SmaliParserContext<'a>,
//         U: 'a + TryDowncast<'a>,
//         T: TryDowncast<'a, Error = U::Error>,
// {
//     type Error = U::Error;
//
//     // #[inline]
//     fn try_downcast_into(self: Rc<Self>) -> Result<Rc<U>, Self::Error> {
//         U::try_downcast_rc(self)
//     }
// }
