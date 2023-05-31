use crate::Allocator;
use num_traits::PrimInt;
use std::ops::AddAssign;
use thiserror::Error;

pub struct SequentialAllocator<T> {
    curr: T,
}

impl<T: PrimInt> SequentialAllocator<T> {
    pub fn new() -> Self {
        Self { curr: T::zero() }
    }
}

impl<T> Allocator for SequentialAllocator<T>
where
    T: PrimInt + AddAssign,
{
    type Id = T;
    type Error = Error;

    fn alloc(&mut self) -> Result<Self::Id, Self::Error> {
        if self.curr == T::max_value() {
            Err(Error::OutOfIds)
        } else {
            let id = self.curr;
            self.curr += T::one();
            Ok(id)
        }
    }

    fn dealloc(&mut self, _: Self::Id) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no more IDs to allocate")]
    OutOfIds,
}
