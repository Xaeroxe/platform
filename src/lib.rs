// Copyright 2018 Jacob Kiesel

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate provides an easy way to inline selection of input parameters
//! based on the platform being targeted.  Can be used on any `Sized` type.
//!
//! This is guaranteed to be a zero cost abstraction, as all calls are inlined.
//!
//! ```
//! extern crate platform;
//!
//! use platform::Platform;
//!
//! fn main() {
//!     println!("Hello from {}!",
//!         "unknown"
//!         .ios("ios")
//!         .android("android")
//!         .windows("windows")
//!         .macos("macos")
//!         .linux("linux")
//!         .wasm32("wasm32")
//!         .emscripten("emscripten")
//!     );
//!
//!     // Alternatively, let's pretend the arguments are non-trivial to evaluate.
//!     // We can also use this on function pointers so long as all the variants can 
//!     // coerce to the same function pointer type.
//!     println!("Hello from {}!",
//!         ((|| String::from("unknown")) as fn() -> String)
//!         .ios(|| String::from("ios"))
//!         .android(|| String::from("android"))
//!         .windows(|| String::from("windows"))
//!         .macos(|| String::from("macos"))
//!         .linux(|| String::from("linux"))
//!         .wasm32(|| String::from("wasm32"))
//!         .emscripten(|| String::from("emscripten"))
//!         ()
//!     );
//! }
//! ```

macro_rules! define_platform {
    ($platform:ident, $name:tt) => {
        #[inline(always)]
        fn $platform(self, _input: Self) -> Self {
            #[cfg(target_os=$name)]
            {
                _input
            }
            #[cfg(not(target_os=$name))]
            {
                self
            }
        }
    };
}

pub trait Platform: Sized {
    #[inline(always)]
    fn wasm32(self, _input: Self) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            _input
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self
        }
    }
    define_platform!(ios, "ios");
    define_platform!(android, "android");
    define_platform!(windows, "windows");
    define_platform!(macos, "macos");
    define_platform!(linux, "linux");
    define_platform!(emscripten, "emscripten");
    define_platform!(freebsd, "freebsd");
    define_platform!(openbsd, "openbsd");
    define_platform!(dragonfly, "dragonfly");
    define_platform!(netbsd, "netbsd");
    define_platform!(redox, "redox");
}

impl<T> Platform for T {}
