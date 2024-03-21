// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

pub mod number;
pub mod vector;

pub use number::*;
pub use vector::*;

pub type BVector2 = Vector2<bool>;
pub type IVector2 = Vector2<i32>;
pub type UVector2 = Vector2<u32>;
pub type FVector2 = Vector2<f32>;
pub type DVector2 = Vector2<f64>;

pub type BVector3 = Vector3<bool>;
pub type IVector3 = Vector3<i32>;
pub type UVector3 = Vector3<u32>;
pub type FVector3 = Vector3<f32>;
pub type DVector3 = Vector3<f64>;

pub type BVector4 = Vector4<bool>;
pub type IVector4 = Vector4<i32>;
pub type UVector4 = Vector4<u32>;
pub type FVector4 = Vector4<f32>;
pub type DVector4 = Vector4<f64>;

#[macro_export]
macro_rules! impl_ops {
    ($typename:ident { $($field:ident),+ }) => {
        impl<T: Any + Send + Sync + Add<Output = T>> Add<$typename<T>> for $typename<T> {
            type Output = $typename<T>;

            fn add(self, other: Self) -> Self {
                Self::new($(self.$field + other.$field),+)
            }
        }

        impl<T: Any + Send + Sync + Sub<Output = T>> Sub<$typename<T>> for $typename<T> {
            type Output = $typename<T>;

            fn sub(self, other: Self) -> Self {
                Self::new($(self.$field - other.$field),+)
            }
        }

        impl<T: Any + Send + Sync + Mul<Output = T>> Mul<$typename<T>> for $typename<T> {
            type Output = $typename<T>;

            fn mul(self, other: Self) -> Self {
                Self::new($(self.$field * other.$field),+)
            }
        }

        impl<T: Any + Send + Sync + Div<Output = T>> Div<$typename<T>> for $typename<T> {
            type Output = $typename<T>;

            fn div(self, other: Self) -> Self {
                Self::new($(self.$field / other.$field),+)
            }
        }

        impl<T: Any + Send + Sync + AddAssign<T>> AddAssign<$typename<T>> for $typename<T> {
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field);+
            }
        }

        impl<T: Any + Send + Sync + SubAssign> SubAssign<$typename<T>> for $typename<T> {
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field);+
            }
        }

        impl<T: Any + Send + Sync + MulAssign> MulAssign<$typename<T>> for $typename<T> {
            fn mul_assign(&mut self, other: Self) {
                $(self.$field *= other.$field);+
            }
        }

        impl<T: Any + Send + Sync + DivAssign> DivAssign<$typename<T>> for $typename<T> {
            fn div_assign(&mut self, other: Self) {
                $(self.$field /= other.$field);+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_ref {
    ($typename:ident, $len:expr) => {
        impl<T: Any + Send + Sync> AsRef<[T; $len]> for $typename<T> {
            #[inline]
            fn as_ref(&self) -> &[T; $len] {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<T: Any + Send + Sync> AsMut<[T; $len]> for $typename<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; $len] {
                unsafe { std::mem::transmute(self) }
            }
        }
    };
}
