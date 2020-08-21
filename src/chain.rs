//! Data structures describing a chain of blocks.
//!
//! A chain of blocks is composed of two parts:
//!
//! - A list of finalized blocks. Finalized blocks are blocks that are forever part of the chain
//! and can never be reverted.
//! - A tree of non-finalized blocks built on top of the last finalized block.
//!
//! When a block first appears, it is verified and, on success, added to the tree of
//! non-finalized blocks. Later, this block might get finalized. When a block is finalized, all
//! the blocks that are not one of its ancestors or descendants is entirely discarded.
//!
//! Example chain:
//!
//! ```ignore
//!                             +-> #5
//!                             |
//!                      +-> #4 +-> #5
//!                      |
//! #0 +> #1 +> #2 +> #3 +-> #4 +-> #5 +> #6
//! ```
//!
//! In this example, #3 is the latest finalized block. Before and including #3, the chain is a
//! simple list. After #3, the chain becomes a tree.

pub mod headers;
pub mod headers_async;