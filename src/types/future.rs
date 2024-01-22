// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use std::future::Future;
use std::pin::Pin;

#[cfg(not(target_arch = "wasm32"))]
pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[cfg(target_arch = "wasm32")]
pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
