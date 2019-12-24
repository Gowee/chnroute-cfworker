use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub trait MathLog2 {
    // We follow the return type convention used in `.leading_zeros`.
    // https://users.rust-lang.org/t/why-the-return-type-of-int-leading-zeros-is-u32-of-u8/
    fn log2(self) -> u32;
}

macro_rules! implement_log2 {
    ($int: ident) => {
        impl MathLog2 for $int {
            fn log2(self) -> u32 {
                // https://users.rust-lang.org/t/logarithm-of-integers/8506/5
                std::mem::size_of::<Self>() as u32 * 8 - self.leading_zeros() - 1
            }
        }
    }
}

implement_log2!(u32);
implement_log2!(u128);

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
