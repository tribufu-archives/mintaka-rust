// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use num_traits::{Num, NumCast};
use std::fmt::Debug;
use std::ops::*;

pub trait Number:
    Copy
    + Clone
    + Debug
    + Num
    + NumCast
    + PartialEq
    + PartialOrd
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
{
}
