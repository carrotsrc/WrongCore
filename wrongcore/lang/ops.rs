// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use ::Sized;

#[lang = "drop"]
pub trait Drop {
    /// The `drop` method, called when the value goes out of scope.
    fn drop(&mut self);
}

// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

#[lang = "add"]
pub trait Add<RHS=Self> {
    /// The resulting type after applying the `+` operator
    type Output;

    /// The method for the `+` operator
    fn add(self, rhs: RHS) -> Self::Output;
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t { self + other }
        }

        forward_ref_binop! { impl Add, add for $t, $t }
    )*)
}

add_impl! { usize u8 u16 u32 isize i8 i16 i32 f32 }

#[lang = "sub"]
pub trait Sub<RHS=Self> {
    /// The resulting type after applying the `-` operator
    type Output;

    /// The method for the `-` operator
    fn sub(self, rhs: RHS) -> Self::Output;
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: $t) -> $t { self - other }
        }

        forward_ref_binop! { impl Sub, sub for $t, $t }
    )*)
}

sub_impl! { usize u8 u16 u32 isize i8 i16 i32 f32 }

#[lang = "mul"]
pub trait Mul<RHS=Self> {
    /// The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}

macro_rules! mul_impl {
    ($($t:ty)*) => ($(

        impl Mul for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $t) -> $t { self * other }
        }

        forward_ref_binop! { impl Mul, mul for $t, $t }
    )*)
}

mul_impl! { usize u8 u16 u32 isize i8 i16 i32 f32 }

#[lang = "div"]
pub trait Div<RHS=Self> {
    /// The resulting type after applying the `/` operator
    type Output;

    /// The method for the `/` operator
    fn div(self, rhs: RHS) -> Self::Output;
}

macro_rules! div_impl {
    ($($t:ty)*) => ($(

        impl Div for $t {
            type Output = $t;

            #[inline]
            fn div(self, other: $t) -> $t { self / other }
        }

        forward_ref_binop! { impl Div, div for $t, $t }
    )*)
}

div_impl! { usize u8 u16 u32 isize i8 i16 i32 f32 }

#[lang = "rem"]
pub trait Rem<RHS=Self> {
    /// The resulting type after applying the `%` operator
    type Output = Self;

    /// The method for the `%` operator
    fn rem(self, rhs: RHS) -> Self::Output;
}

macro_rules! rem_impl {
    ($($t:ty)*) => ($(
        impl Rem for $t {
            type Output = $t;

            #[inline]
            fn rem(self, other: $t) -> $t { self % other }
        }

        forward_ref_binop! { impl Rem, rem for $t, $t }
    )*)
}


rem_impl! { usize u8 u16 u32 isize i8 i16 i32 }

#[lang = "neg"]
pub trait Neg {
    /// The resulting type after applying the `-` operator
    type Output;

    /// The method for the unary `-` operator
    fn neg(self) -> Self::Output;
}



macro_rules! neg_impl_core {
    ($id:ident => $body:expr, $($t:ty)*) => ($(
        #[allow(unsigned_negation)]
        impl Neg for $t {
            type Output = $t;

            #[inline]
            fn neg(self) -> $t { let $id = self; $body }
        }

        forward_ref_unop! { impl Neg, neg for $t }
    )*)
}

macro_rules! neg_impl_numeric {
    ($($t:ty)*) => { neg_impl_core!{ x => -x, $($t)*} }
}

macro_rules! neg_impl_unsigned {
    ($($t:ty)*) => {
        neg_impl_core!{ x => {
            !x.wrapping_add(1)
        }, $($t)*} }
}

// neg_impl_unsigned! { usize u8 u16 u32 }
neg_impl_numeric! { isize i8 i16 i32 f32 }

#[lang = "not"]
pub trait Not {
    /// The resulting type after applying the `!` operator
    type Output;

    /// The method for the unary `!` operator
    fn not(self) -> Self::Output;
}

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        impl Not for $t {
            type Output = $t;

            #[inline]
            fn not(self) -> $t { !self }
        }

        forward_ref_unop! { impl Not, not for $t }
    )*)
}

not_impl! { bool usize u8 u16 u32 isize i8 i16 i32 }

#[lang = "bitand"]
pub trait BitAnd<RHS=Self> {
    /// The resulting type after applying the `&` operator
    type Output;

    /// The method for the `&` operator
    fn bitand(self, rhs: RHS) -> Self::Output;
}

macro_rules! bitand_impl {
    ($($t:ty)*) => ($(
        impl BitAnd for $t {
            type Output = $t;

            #[inline]
            fn bitand(self, rhs: $t) -> $t { self & rhs }
        }

        forward_ref_binop! { impl BitAnd, bitand for $t, $t }
    )*)
}

bitand_impl! { bool usize u8 u16 u32 isize i8 i16 i32  }

#[lang = "bitor"]
pub trait BitOr<RHS=Self> {
    /// The resulting type after applying the `|` operator
    type Output;

    /// The method for the `|` operator
    fn bitor(self, rhs: RHS) -> Self::Output;
}

macro_rules! bitor_impl {
    ($($t:ty)*) => ($(
        impl BitOr for $t {
            type Output = $t;

            #[inline]
            fn bitor(self, rhs: $t) -> $t { self | rhs }
        }

        forward_ref_binop! { impl BitOr, bitor for $t, $t }
    )*)
}

bitor_impl! { bool usize u8 u16 u32 isize i8 i16 i32 }

#[lang = "bitxor"]
pub trait BitXor<RHS=Self> {
    /// The resulting type after applying the `^` operator
    type Output;

    /// The method for the `^` operator
    fn bitxor(self, rhs: RHS) -> Self::Output;
}

macro_rules! bitxor_impl {
    ($($t:ty)*) => ($(
        impl BitXor for $t {
            type Output = $t;

            #[inline]
            fn bitxor(self, other: $t) -> $t { self ^ other }
        }

        forward_ref_binop! { impl BitXor, bitxor for $t, $t }
    )*)
}

bitxor_impl! { bool usize u8 u16 u32 isize i8 i16 i32 }

#[lang = "shl"]
pub trait Shl<RHS> {
    /// The resulting type after applying the `<<` operator
    type Output;

    /// The method for the `<<` operator
    fn shl(self, rhs: RHS) -> Self::Output;
}

macro_rules! shl_impl {
    ($t:ty, $f:ty) => (
        impl Shl<$f> for $t {
            type Output = $t;

            #[inline]
            fn shl(self, other: $f) -> $t {
                self << other
            }
        }

        forward_ref_binop! { impl Shl, shl for $t, $f }
    )
}

macro_rules! shl_impl_all {
    ($($t:ty)*) => ($(
        shl_impl! { $t, u8 }
        shl_impl! { $t, u16 }
        shl_impl! { $t, u32 }
        shl_impl! { $t, usize }

        shl_impl! { $t, i8 }
        shl_impl! { $t, i16 }
        shl_impl! { $t, i32 }
        shl_impl! { $t, isize }
    )*)
}

shl_impl_all! { u8 u16 u32 usize i8 i16 i32 isize }

#[lang = "shr"]
pub trait Shr<RHS> {
    /// The resulting type after applying the `>>` operator
    type Output;

    /// The method for the `>>` operator
    fn shr(self, rhs: RHS) -> Self::Output;
}

macro_rules! shr_impl {
    ($t:ty, $f:ty) => (
        impl Shr<$f> for $t {
            type Output = $t;

            #[inline]
            fn shr(self, other: $f) -> $t {
                self >> other
            }
        }

        forward_ref_binop! { impl Shr, shr for $t, $f }
    )
}

macro_rules! shr_impl_all {
    ($($t:ty)*) => ($(
        shr_impl! { $t, u8 }
        shr_impl! { $t, u16 }
        shr_impl! { $t, u32 }
        shr_impl! { $t, usize }

        shr_impl! { $t, i8 }
        shr_impl! { $t, i16 }
        shr_impl! { $t, i32 }
        shr_impl! { $t, isize }
    )*)
}

shr_impl_all! { u8 u16 u32 usize i8 i16 i32 isize }

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    /// The returned type after indexing
    type Output: ?Sized;

    /// The method for the indexing (`Foo[Bar]`) operation
    fn index<'a>(&'a self, index: Idx) -> &'a Self::Output;
}

#[lang = "index_mut"]
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    /// The method for the indexing (`Foo[Bar]`) operation
    fn index_mut<'a>(&'a mut self, index: Idx) -> &'a mut Self::Output;
}

/*
/// An unbounded range.
#[derive(Copy, Clone, PartialEq, Eq)]
#[lang = "range_full"]
pub struct RangeFull;

impl fmt::Debug for RangeFull {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "..")
    }
}

/// A (half-open) range which is bounded at both ends.
#[derive(Clone, PartialEq, Eq)]
#[lang = "range"]
pub struct Range<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: Idx,
    /// The upper bound of the range (exclusive).
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}..{:?}", self.start, self.end)
    }
}

/// A range which is only bounded below.
#[derive(Clone, PartialEq, Eq)]
#[lang = "range_from"]
pub struct RangeFrom<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for RangeFrom<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}..", self.start)
    }
}

/// A range which is only bounded above.
#[derive(Copy, Clone, PartialEq, Eq)]
#[lang = "range_to"]
pub struct RangeTo<Idx> {
    /// The upper bound of the range (exclusive).
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for RangeTo<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "..{:?}", self.end)
    }
}
*/


#[lang = "deref"]
pub trait Deref {
    /// The resulting type after dereferencing
    type Target: ?Sized;

    /// The method called to dereference a value
    fn deref<'a>(&'a self) -> &'a Self::Target;
}

impl<'a, T: ?Sized> Deref for &'a T {
    type Target = T;

    fn deref(&self) -> &T { *self }
}

impl<'a, T: ?Sized> Deref for &'a mut T {
    type Target = T;

    fn deref(&self) -> &T { *self }
}

#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    /// The method called to mutably dereference a value
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target;
}

impl<'a, T: ?Sized> DerefMut for &'a mut T {
    fn deref_mut(&mut self) -> &mut T { *self }
}
