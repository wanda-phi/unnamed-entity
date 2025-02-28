use core::hash::Hash;
use core::marker::PhantomData;

use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! make_res_type {
    ($t:ident, $b:ty, $i:ty) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(
            feature = "serde",
            derive(Serialize, Deserialize),
            serde(into = "usize"),
            serde(try_from = "usize")
        )]
        pub struct $t($b);

        impl $t {
            pub const fn from_usize_const(val: usize) -> Self {
                let tmp = (val + 1) as $i;
                $t(<$b>::new(tmp).unwrap())
            }
        }

        impl From<$t> for usize {
            fn from(x: $t) -> usize {
                (x.0.get() - 1).try_into().unwrap()
            }
        }

        impl TryFrom<usize> for $t {
            type Error = std::num::TryFromIntError;
            fn try_from(x: usize) -> Result<Self, Self::Error> {
                let tmp: $i = (x + 1).try_into()?;
                Ok($t(tmp.try_into()?))
            }
        }

        impl std::fmt::Debug for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> core::result::Result<(), std::fmt::Error> {
                write!(f, "{}", usize::from(*self))
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> core::result::Result<(), std::fmt::Error> {
                write!(f, "{}", usize::from(*self))
            }
        }
    };
}

make_res_type!(__ReservedU8, core::num::NonZeroU8, u8);
make_res_type!(__ReservedU16, core::num::NonZeroU16, u16);
make_res_type!(__ReservedU32, core::num::NonZeroU32, u32);
make_res_type!(__ReservedUsize, core::num::NonZeroUsize, usize);

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __def_entity_id {
    ($v:vis, $id:ident, $t:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, $crate::__serde::Serialize, $crate::__serde::Deserialize)]
        #[serde(crate="unnamed_entity::__serde")]
        $v struct $id($t);
    }
}

#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! __def_entity_id {
    ($v:vis, $id:ident, $t:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        $v struct $id($t);
    }
}

#[macro_export]
macro_rules! __impl_entity_id {
    ($v:vis, $id:ident, $t:ty) => {
        $crate::__def_entity_id!($v, $id, $t);

        impl $crate::EntityId for $id {
            fn to_idx(self) -> usize {
                self.0.try_into().unwrap()
            }
            fn from_idx(idx: usize) -> Self {
                Self(idx.try_into().unwrap())
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> core::result::Result<(), std::fmt::Error> {
                use $crate::EntityId;
                write!(f, "{}", self.to_idx())
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_entity_id_const {
    ($v:vis, $id:ident, $t:ty) => {
        impl $id {
            pub const fn from_idx_const(val: usize) -> Self {
                Self(val as $t)
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_entity_id_const_reserved {
    ($v:vis, $id:ident, $t:ty) => {
        impl $id {
            pub const fn from_idx_const(val: usize) -> Self {
                Self(<$t>::from_usize_const(val))
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_entity_id_delta {
    ($v:vis, $id:ident, $t:ty) => {
        impl core::ops::Add<usize> for $id {
            type Output = $id;
            fn add(self, x: usize) -> Self {
                use $crate::EntityId;
                Self::from_idx(self.to_idx() + x)
            }
        }

        impl core::ops::AddAssign<usize> for $id {
            fn add_assign(&mut self, x: usize) {
                *self = *self + x;
            }
        }

        impl core::ops::Sub<usize> for $id {
            type Output = $id;
            fn sub(self, x: usize) -> Self {
                use $crate::EntityId;
                Self::from_idx(self.to_idx() - x)
            }
        }

        impl core::ops::SubAssign<usize> for $id {
            fn sub_assign(&mut self, x: usize) {
                *self = *self - x;
            }
        }

        impl core::ops::Add<isize> for $id {
            type Output = $id;
            fn add(self, x: isize) -> Self {
                use $crate::EntityId;
                Self::from_idx(self.to_idx().checked_add_signed(x).unwrap())
            }
        }

        impl core::ops::AddAssign<isize> for $id {
            fn add_assign(&mut self, x: isize) {
                *self = *self + x;
            }
        }

        impl core::ops::Sub<isize> for $id {
            type Output = $id;
            fn sub(self, x: isize) -> Self {
                self + (-x)
            }
        }

        impl core::ops::SubAssign<isize> for $id {
            fn sub_assign(&mut self, x: isize) {
                *self = *self - x;
            }
        }

        impl core::ops::Add<i32> for $id {
            type Output = $id;
            fn add(self, x: i32) -> Self {
                self + (x as isize)
            }
        }

        impl core::ops::AddAssign<i32> for $id {
            fn add_assign(&mut self, x: i32) {
                *self = *self + (x as isize);
            }
        }

        impl core::ops::Sub<i32> for $id {
            type Output = $id;
            fn sub(self, x: i32) -> Self {
                self - (x as isize)
            }
        }

        impl core::ops::SubAssign<i32> for $id {
            fn sub_assign(&mut self, x: i32) {
                *self = *self - (x as isize);
            }
        }

        impl core::ops::Sub<$id> for $id {
            type Output = isize;
            fn sub(self, x: Self) -> isize {
                use $crate::EntityId;
                self.to_idx() as isize - x.to_idx() as isize
            }
        }

        impl $id {
            pub fn range(self, other: $id) -> $crate::id::EntityIds<$id> {
                use $crate::EntityId;
                $crate::id::EntityIds::new_range(self.to_idx(), other.to_idx())
            }
        }
    };
}

#[macro_export]
macro_rules! __reserved_ty {
    (u8) => {
        $crate::id::__ReservedU8
    };
    (u16) => {
        $crate::id::__ReservedU16
    };
    (u32) => {
        $crate::id::__ReservedU32
    };
    (usize) => {
        $crate::id::__ReservedUsize
    };
}

#[macro_export]
macro_rules! entity_id {
    ($v:vis id $id:ident $t:ty; $($rest:tt)*) => {
        $crate::__impl_entity_id!($v, $id, $t);
        $crate::__impl_entity_id_const!($v, $id, $t);
        $crate::entity_id!{ $($rest)* }
    };
    ($v:vis id $id:ident $t:ty, delta; $($rest:tt)*) => {
        $crate::__impl_entity_id!($v, $id, $t);
        $crate::__impl_entity_id_const!($v, $id, $t);
        $crate::__impl_entity_id_delta!($v, $id, $t);
        $crate::entity_id!{ $($rest)* }
    };
    ($v:vis id $id:ident $t:tt, reserve 1; $($rest:tt)*) => {
        $crate::__impl_entity_id!($v, $id, $crate::__reserved_ty!($t));
        $crate::__impl_entity_id_const_reserved!($v, $id, $crate::__reserved_ty!($t));
        $crate::entity_id!{ $($rest)* }
    };
    ($v:vis id $id:ident $t:tt, reserve 1, delta; $($rest:tt)*) => {
        $crate::__impl_entity_id!($v, $id, $crate::__reserved_ty!($t));
        $crate::__impl_entity_id_delta!($v, $id, $t);
        $crate::__impl_entity_id_const_reserved!($v, $id, $crate::__reserved_ty!($t));
        $crate::entity_id!{ $($rest)* }
    };
    () => {};
}

#[cfg(feature = "serde")]
pub trait EntityId:
    Debug
    + Copy
    + Clone
    + Send
    + Sync
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash
    + Serialize
    + for<'de> Deserialize<'de>
{
    fn from_idx(idx: usize) -> Self;
    fn to_idx(self) -> usize;
}

#[cfg(not(feature = "serde"))]
pub trait EntityId:
    Debug + Copy + Clone + Send + Sync + Eq + PartialEq + Ord + PartialOrd + Hash
{
    fn from_idx(idx: usize) -> Self;
    fn to_idx(self) -> usize;
}

// iterator

#[derive(Clone, Debug)]
pub struct EntityIds<I: EntityId> {
    start: usize,
    end: usize,
    ids: PhantomData<I>,
}

impl<I: EntityId> EntityIds<I> {
    pub fn new(num: usize) -> Self {
        Self {
            start: 0,
            end: num,
            ids: PhantomData,
        }
    }

    pub fn new_range(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self {
            start,
            end,
            ids: PhantomData,
        }
    }
}

impl<I: EntityId> Iterator for EntityIds<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        if self.start == self.end {
            None
        } else {
            let res = I::from_idx(self.start);
            self.start += 1;
            Some(res)
        }
    }
}

impl<I: EntityId> DoubleEndedIterator for EntityIds<I> {
    fn next_back(&mut self) -> Option<I> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(I::from_idx(self.end))
        }
    }
}

impl<I: EntityId> ExactSizeIterator for EntityIds<I> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}
