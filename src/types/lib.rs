// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use type_uuid::TypeUuid;
pub use type_uuid::TypeUuidDynamic;
pub use uuid::Uuid;

pub mod arch;
pub mod future;
pub mod platform;

pub type TypeUuidBytes = type_uuid::Bytes;
