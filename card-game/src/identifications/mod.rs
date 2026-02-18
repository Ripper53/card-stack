mod action;
mod card;
mod player;

pub use action::*;
pub use card::*;
pub use player::*;
use state_validation::{StateFilterInputCombination, StateFilterInputConversion};

pub trait CastTo<T> {
    fn cast_ref(&self) -> &T;
}

/// ID that allows mutation.
///
/// Ex. `ValidCardID` -> `MutID<ValidCardID>`
/// can be used to retrieve a mutable reference to a card
#[derive(Debug)]
pub struct MutID<ID>(ID);
impl<ID> MutID<ID> {
    pub fn new(id: ID) -> Self {
        MutID(id)
    }
    pub fn id(&self) -> &ID {
        &self.0
    }
    pub fn take_id(self) -> ID {
        self.0
    }
    pub fn into_id<NewID>(self) -> MutID<NewID>
    where
        ID: Into<NewID>,
    {
        MutID(self.0.into())
    }
}
pub trait IterMutID<ID> {
    fn iter_mut_id(self) -> impl Iterator<Item = MutID<ID>>;
}
impl<ID> IterMutID<ID> for MutID<Vec<ID>> {
    fn iter_mut_id(self) -> impl Iterator<Item = MutID<ID>> {
        self.0.into_iter().map(|id| MutID::new(id))
    }
}
macro_rules! mut_id_for_tuple {
    ($($index_name: tt => $id: ident),*) => {
        impl<$($id),*> MutID<($($id),*)> {
            pub fn split_take(self) -> ($(MutID<$id>),*) {
                ($(
                    MutID::new(self.0.$index_name)
                ),*)
            }
        }
    };
}
mut_id_for_tuple!(0 => T0, 1 => T1);
mut_id_for_tuple!(0 => T0, 1 => T1, 2 => T2);
mut_id_for_tuple!(0 => T0, 1 => T1, 2 => T2, 3 => T3);
impl<T: CastTo<CastedT>, CastedT> CastTo<CastedT> for MutID<T> {
    fn cast_ref(&self) -> &CastedT {
        self.0.cast_ref()
    }
}
impl<T: UncheckedReplaceFilter> UncheckedReplaceFilter for MutID<T> {
    type Output<F> = MutID<T::Output<F>>;
    fn unchecked_replace_filter<F>(self) -> Self::Output<F> {
        MutID::new(self.0.unchecked_replace_filter())
    }
}
impl<T: UncheckedClone> UncheckedClone for MutID<T> {
    fn unchecked_clone(&self) -> Self {
        MutID::new(self.0.unchecked_clone())
    }
}
#[derive(Debug)]
pub struct MutIDRemainder<ID>(std::marker::PhantomData<ID>);
impl<ID> StateFilterInputConversion<ID> for MutID<ID> {
    type Remainder = MutIDRemainder<ID>;
    fn split_take(self) -> (ID, Self::Remainder) {
        (self.0, MutIDRemainder(std::marker::PhantomData::default()))
    }
}
/*impl<T: StateFilterInputConversion<ID>, ID> StateFilterInputConversion<T>
    for MutID<ID, T::Remainder>
{
    type Remainder = MutIDRemainder<ID>;
    fn split_take(self) -> (ID, Self::Remainder) {
        (self.0, MutIDRemainder(std::marker::PhantomData::default()))
    }
}*/
impl<NewID, OldID> StateFilterInputCombination<NewID> for MutIDRemainder<OldID> {
    type Combined = MutID<NewID>;
    fn combine(self, new_id: NewID) -> Self::Combined {
        MutID::new(new_id)
    }
}
/*impl<T: StateFilterInputConversion<ID>, ID> StateFilterInputCombination<T> for MutIDRemainder<ID>
where
    T::Remainder: StateFilterInputCombination<MutID<ID>>,
{
    type Combined = <T::Remainder as StateFilterInputCombination<MutID<ID>>>::Combined;
    fn combine(self, value: T) -> Self::Combined {
        let (id, remainder) = value.split_take();
        remainder.combine(MutID::new(id))
    }
}*/

pub trait UncheckedReplaceFilter {
    type Output<F>;
    fn unchecked_replace_filter<F>(self) -> Self::Output<F>;
}

pub trait UncheckedClone {
    fn unchecked_clone(&self) -> Self;
}

pub trait FilterSupertype<T> {}
//impl<T: FilterSupertype<IntoT>, IntoT> FilterSupertype<(T,)> for (IntoT,) {}
//impl<T> FilterSupertype<()> for T {}
macro_rules! impl_filter_supertype_for_tuple {
    ($($list_ty: ident),* | |) => {};
    ($($list_ty: ident),* | $first_into_ty: ident $(,)? | $($lost_ty: ident),*) => {};
    ($($list_ty: ident),* | $first_into_ty: ident, $second_into_ty: ident $(,)? | $($lost_ty: ident),*) => {};
    ($first_ty: ident $(, $list_ty: ident)+ $(,)? | $first_into_ty: ident $(, $list_into_ty: ident)+ | $($lost_ty: ident),*) => {
        impl<$($list_ty: FilterSupertype<$list_into_ty>,)* $($lost_ty,)* $($list_into_ty,)* $first_ty> FilterSupertype<($($list_into_ty),*)> for ($($list_ty,)* $($lost_ty,)* $first_ty) {}
        impl_filter_supertype_for_tuple!($($list_ty),* | $($list_into_ty),* | $($lost_ty,)* $first_ty);
    };
    ($($list_ty: ident),+ | $($list_into_ty: ident),+ $(,)?) => {
        impl<$($list_ty: FilterSupertype<$list_into_ty>,)* $($list_into_ty),*> FilterSupertype<($($list_into_ty),*)> for ($($list_ty),*) {}
        impl_filter_supertype_for_tuple!($($list_ty),* | $($list_into_ty),* |);
    };
}
impl_filter_supertype_for_tuple!(T0, T1 | IntoT0, IntoT1);
impl_filter_supertype_for_tuple!(T0, T1, T2 | IntoT0, IntoT1, IntoT2);
impl_filter_supertype_for_tuple!(T0, T1, T2, T3 | IntoT0, IntoT1, IntoT2, IntoT3);
impl_filter_supertype_for_tuple!(T0, T1, T2, T3, T4 | IntoT0, IntoT1, IntoT2, IntoT3, IntoT4);
impl_filter_supertype_for_tuple!(
    T0,
    T1,
    T2,
    T3,
    T4,
    T5 | IntoT0,
    IntoT1,
    IntoT2,
    IntoT3,
    IntoT4,
    IntoT5,
);
impl_filter_supertype_for_tuple!(
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6 | IntoT0,
    IntoT1,
    IntoT2,
    IntoT3,
    IntoT4,
    IntoT5,
    IntoT6,
);
impl_filter_supertype_for_tuple!(
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7 | IntoT0,
    IntoT1,
    IntoT2,
    IntoT3,
    IntoT4,
    IntoT5,
    IntoT6,
    IntoT7,
);

/*
impl<
    T0: FilterSupertype<IntoT0>,
    T1: FilterSupertype<IntoT1>,
    T2: FilterSupertype<IntoT2>,
    IntoT0,
    IntoT1,
    IntoT2,
> FilterSupertype<(T0, T1, T2)> for (IntoT0, IntoT1, IntoT2)
{
}
impl<
    T0: FilterSupertype<IntoT0>,
    T1: FilterSupertype<IntoT1>,
    T2: FilterSupertype<IntoT2>,
    T3: FilterSupertype<IntoT3>,
    IntoT0,
    IntoT1,
    IntoT2,
    IntoT3,
> FilterSupertype<(T0, T1, T2, T3)> for (IntoT0, IntoT1, IntoT2, IntoT3)
{
}*/

#[macro_export]
macro_rules! create_valid_identification {
    ($name: ident, $internal_id: ty, with_copy) => {
        create_valid_identification!($name, $internal_id, core);
        impl<F> $name<F> {
            pub fn id(&self) -> $internal_id {
                self.0
            }
        }
    };
    ($name: ident, $internal_id: ty, with_clone) => {
        create_valid_identification!($name, $internal_id, core);
        impl<F> $name<F> {
            pub fn id(&self) -> $internal_id {
                self.0.clone()
            }
        }
    };
    ($name: ident, $internal_id: ty) => {
        create_valid_identification!($name, $internal_id, core);
        impl<F> $name<F> {
            pub fn id(&self) -> &$internal_id {
                &self.0
            }
        }
    };
    ($name: ident, $internal_id: ty, core) => {
        pub struct $name<F>($internal_id, ::std::marker::PhantomData<(F, *const ())>);
        impl<F> std::fmt::Debug for $name<F>
        where
            $internal_id: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Valid({:?})", &self.0)
            }
        }
        impl<F> card_game::stack::NonEmptyInput for $name<F> {}
        impl<F> card_game::identifications::UncheckedReplaceFilter for $name<F> {
            type Output<NewF> = $name<NewF>;
            fn unchecked_replace_filter<NewF>(self) -> Self::Output<NewF> {
                $name(self.0, ::std::marker::PhantomData::default())
            }
        }
        impl<F> card_game::identifications::UncheckedClone for $name<F> {
            fn unchecked_clone(&self) -> Self {
                Self(self.0.clone(), ::std::marker::PhantomData::default())
            }
        }
        impl<F> $name<F> {
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
            pub fn into_filter<NewF>(self) -> $name<NewF>
            where
                F: card_game::identifications::FilterSupertype<NewF>,
            {
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
