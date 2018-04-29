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
//! }
//! ```

pub trait Platform: Sized {
    #[inline(always)]
    fn ios(self, _input: Self) -> Self {
        #[cfg(target_os = "ios")]
        {
            _input
        }

        #[cfg(not(target_os = "ios"))]
        {
            self
        }
    }
    #[inline(always)]
    fn android(self, _input: Self) -> Self {
        #[cfg(target_os = "android")]
        {
            _input
        }

        #[cfg(not(target_os = "android"))]
        {
            self
        }
    }
    #[inline(always)]
    fn windows(self, _input: Self) -> Self {
        #[cfg(target_os = "windows")]
        {
            _input
        }

        #[cfg(not(target_os = "windows"))]
        {
            self
        }
    }
    #[inline(always)]
    fn macos(self, _input: Self) -> Self {
        #[cfg(target_os = "macos")]
        {
            _input
        }

        #[cfg(not(target_os = "macos"))]
        {
            self
        }
    }
    #[inline(always)]
    fn linux(self, _input: Self) -> Self {
        #[cfg(target_os = "linux")]
        {
            _input
        }

        #[cfg(not(target_os = "linux"))]
        {
            self
        }
    }
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
    #[inline(always)]
    fn emscripten(self, _input: Self) -> Self {
        #[cfg(target_os = "emscripten")]
        {
            _input
        }

        #[cfg(not(target_os = "emscripten"))]
        {
            self
        }
    }
    #[inline(always)]
    fn freebsd(self, _input: Self) -> Self {
        #[cfg(target_os = "freebsd")]
        {
            _input
        }

        #[cfg(not(target_os = "freebsd"))]
        {
            self
        }
    }
    #[inline(always)]
    fn openbsd(self, _input: Self) -> Self {
        #[cfg(target_os = "openbsd")]
        {
            _input
        }

        #[cfg(not(target_os = "openbsd"))]
        {
            self
        }
    }
    #[inline(always)]
    fn dragonfly(self, _input: Self) -> Self {
        #[cfg(target_os = "dragonfly")]
        {
            _input
        }

        #[cfg(not(target_os = "dragonfly"))]
        {
            self
        }
    }
    #[inline(always)]
    fn netbsd(self, _input: Self) -> Self {
        #[cfg(target_os = "netbsd")]
        {
            _input
        }

        #[cfg(not(target_os = "netbsd"))]
        {
            self
        }
    }
}

impl<T> Platform for T {}
