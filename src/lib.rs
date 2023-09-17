mod rolling;

#[cfg(feature = "dev")]
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
mod xoodoo_gene;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
mod xoodoo_x86;
#[cfg(not(feature = "dev"))]
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
mod xoodoo_gene;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
mod xoodoo_x86;



mod xoofff;
pub use crate::xoofff::Xoofff;

#[cfg(test)]
mod tests;
