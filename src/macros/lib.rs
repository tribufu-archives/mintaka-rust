// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
