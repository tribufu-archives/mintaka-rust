// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::Number;
use crate::{impl_as_ref, impl_ops};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::ops::*;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vector2<T: Any + Send + Sync> {
    pub x: T,
    pub y: T,
}

impl<T: Any + Send + Sync> Vector2<T> {
    #[inline]
    pub fn extend(self, z: T) -> Vector3<T> {
        Vector3::new(self.x, self.y, z)
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vector3<T: Any + Send + Sync> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Any + Send + Sync> Vector3<T> {
    #[inline]
    pub fn extend(self, w: T) -> Vector4<T> {
        Vector4::new(self.x, self.y, self.z, w)
    }

    #[inline]
    pub fn truncate(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vector4<T: Any + Send + Sync> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Any + Send + Sync> Vector4<T> {
    #[inline]
    pub fn truncate(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }
}

macro_rules! impl_vector {
    ($vector:ident { $($field:ident),+ }, $len:expr) => {
        impl<T: Any + Send + Sync> $vector<T> {
            #[inline]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }

            #[inline]
            pub fn map<F, U: Any + Send + Sync>(self, mut f: F) -> $vector<U>
            where
                F: FnMut(T) -> U
            {
                $vector { $($field: f(self.$field)),+ }
            }
        }

        impl<T: Any + Send + Sync + PartialEq> $vector<T> {
            #[inline]
            pub fn equals(&self, other: $vector<T>) -> bool
            {
                $(self.$field == other.$field)&&+
            }
        }

        impl<T: Any + Send + Sync + Number> $vector<T> {
            #[inline]
            pub fn add(&mut self, $($field: T),+)
            {
                $(self.$field += $field);+
            }

            #[inline]
            pub fn subtract(&mut self, $($field: T),+)
            {
                $(self.$field -= $field);+
            }

            #[inline]
            pub fn divide(&mut self, $($field: T),+)
            {
                $(self.$field /= $field);+
            }

            #[inline]
            pub fn multiply(&mut self, $($field: T),+)
            {
                $(self.$field *= $field);+
            }
        }

        impl<T: Any + Send + Sync> Index<usize> for $vector<T> {
            type Output = T;

            #[inline]
            fn index<'a>(&'a self, i: usize) -> &'a T {
                let v: &[T; $len] = self.as_ref();
                &v[i]
            }
        }

        impl<T: Any + Send + Sync> IndexMut<usize> for $vector<T> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
                let v: &mut [T; $len] = self.as_mut();
                &mut v[i]
            }
        }

        impl_as_ref!($vector, $len);
        impl_ops!($vector { $($field),+ });
    };
}

impl_vector!(Vector2 { x, y }, 2);
impl_vector!(Vector3 { x, y, z }, 3);
impl_vector!(Vector4 { x, y, z, w }, 4);

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn extend() {
        let vec2 = Vector2::<i32>::new(1, 2);
        let vec3 = vec2.extend(3);
        let vec4 = vec3.extend(4);
        assert_eq!(vec4.x, 1);
    }

    #[test]
    fn truncate() {
        let vec4 = Vector4::<i32>::new(1, 2, 3, 4);
        let vec3 = vec4.truncate();
        let vec2 = vec3.truncate();
        assert_eq!(vec2.x, 1);
    }
}
