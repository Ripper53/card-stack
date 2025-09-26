mod card;
mod player;

pub use card::*;
pub use player::*;

pub trait CastTo<T> {
    fn cast_ref(&self) -> &T;
}

#[macro_export]
macro_rules! create_valid_identification {
    ($name: ident, $internal_id: ty) => {
        #[derive(card_game::StateFilterInput, Debug)]
        pub struct $name<F>($internal_id, ::std::marker::PhantomData<(F, *const ())>);
        impl<F> $name<F> {
            pub fn id(&self) -> $internal_id {
                self.0
            }
            pub fn unchecked_replace_filter<NewF>(self) -> $name<NewF> {
                $name(self.0, ::std::marker::PhantomData::default())
            }
            pub fn get<T>(
                &self,
                f: impl ::std::ops::FnOnce(&Self) -> ::std::option::Option<&T>,
            ) -> &T {
                f(self).unwrap()
            }
            pub fn get_mut<T>(
                &mut self,
                f: impl ::std::ops::FnOnce(&Self) -> ::std::option::Option<&mut T>,
            ) -> &mut T {
                f(self).unwrap()
            }
            pub fn remove<T>(
                &self,
                f: impl ::std::ops::FnOnce(&Self) -> ::std::option::Option<T>,
            ) -> T {
                f(self).unwrap()
            }
            pub fn unchecked_clone(&self) -> Self {
                $name(self.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F> ::std::convert::From<$name<F>> for $internal_id {
            fn from(valid_id: $name<F>) -> Self {
                valid_id.0
            }
        }
        impl<F0, F1> ::std::convert::From<$name<(F0, F1)>> for $name<F0> {
            fn from(valid_id: $name<(F0, F1)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2> ::std::convert::From<$name<(F0, F1, F2)>> for $name<(F0, F1)> {
            fn from(valid_id: $name<(F0, F1, F2)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2, F3> ::std::convert::From<$name<(F0, F1, F2, F3)>> for $name<(F0, F1, F2)> {
            fn from(valid_id: $name<(F0, F1, F2, F3)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2, F3, F4> ::std::convert::From<$name<(F0, F1, F2, F3, F4)>>
            for $name<(F0, F1, F2, F3)>
        {
            fn from(valid_id: $name<(F0, F1, F2, F3, F4)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2, F3, F4, F5> ::std::convert::From<$name<(F0, F1, F2, F3, F4, F5)>>
            for $name<(F0, F1, F2, F3, F4)>
        {
            fn from(valid_id: $name<(F0, F1, F2, F3, F4, F5)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2, F3, F4, F5, F6> ::std::convert::From<$name<(F0, F1, F2, F3, F4, F5, F6)>>
            for $name<(F0, F1, F2, F3, F4, F5)>
        {
            fn from(valid_id: $name<(F0, F1, F2, F3, F4, F5, F6)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1, F2, F3, F4, F5, F6, F7>
            ::std::convert::From<$name<(F0, F1, F2, F3, F4, F5, F6, F7)>>
            for $name<(F0, F1, F2, F3, F4, F5, F6)>
        {
            fn from(valid_id: $name<(F0, F1, F2, F3, F4, F5, F6, F7)>) -> Self {
                Self(valid_id.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F0, F1> card_game::identifications::CastTo<$name<F0>> for $name<(F0, F1)> {
            fn cast_ref(&self) -> &$name<F0> {
                // SAFETY: only type changing the phantom type
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<F0, F1, F2> card_game::identifications::CastTo<$name<(F0, F1)>>
            for $name<(F0, F1, F2)>
        {
            fn cast_ref(&self) -> &$name<(F0, F1)> {
                // SAFETY: only type changing the phantom type
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<F0, F1, F2, F3> card_game::identifications::CastTo<$name<(F0, F1, F2)>>
            for $name<(F0, F1, F2, F3)>
        {
            fn cast_ref(&self) -> &$name<(F0, F1, F2)> {
                // SAFETY: only type changing the phantom type
                unsafe { ::std::mem::transmute(self) }
            }
        }

        impl<F> ::std::cmp::PartialEq for $name<F> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        impl<F> ::std::cmp::Eq for $name<F> {}
        impl<F> ::std::hash::Hash for $name<F> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }
    };
}
