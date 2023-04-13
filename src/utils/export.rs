#[doc(hidden)]
#[macro_export]
macro_rules! export {
    ($mod:ident, $path:literal) => {
        #[path=$path]
        mod $mod;
        pub use $crate::$mod::*;
    };
}
