/// An `Allocator` oversees the allocation and deallocation of IDs.
pub trait Allocator {
    /// The type of ID.
    type Id;
    /// Associated error type.
    type Error: std::error::Error + core::fmt::Debug;

    /// Allocates an ID.
    fn alloc(&mut self) -> Result<Self::Id, Self::Error>;

    /// Deallocates an ID.
    fn dealloc(&mut self, id: Self::Id) -> Result<(), Self::Error>;

    /// Reserves an ID.
    fn reserve(&mut self, id: Self::Id) -> Result<(), Self::Error>;
}

#[cfg(feature = "seq")]
pub mod seq;
