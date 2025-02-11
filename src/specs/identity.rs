/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::core::morphism::Morphism;

pub trait Identity: Morphism {
    fn id<A>() -> Self::T<A, A>;
}
