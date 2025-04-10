// Declare the `mobs` module, which is likely defined in a separate file
mod mobs;

// Re-export everything from the `mobs` module, making its contents available at the current module level
// This allows other parts of the code to access items from `mobs` without explicitly importing it
pub use mobs::*;