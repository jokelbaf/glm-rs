//
// GLSL Mathematics for Rust.
//
// Copyright (c) 2015 The glm-rs authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//!
//! Built-in funcions defined in GLSL specification chapter 8.
//!
//!

pub use self::trig::{
    acos, acosh, asin, asinh, atan, atan2, atanh, cos, cosh, degrees, radians, sin, sinh, tan, tanh,
};

pub use self::exp::{exp, exp2, inversesqrt, log, log2, pow, sqrt};

pub use self::common::{
    abs, ceil, clamp, clamp_s, floatBitsToInt, floatBitsToUint, floor, fma, fmod, fract, frexp,
    intBitsToFloat, isinf, isnan, ldexp, max, max_s, min, min_s, mix, mix_bool, mix_s, mod_s, modf,
    round, roundEven, sign, smoothstep, smoothstep_s, step, step_s, trunc, uintBitsToFloat,
};

pub use self::pack::{
    packDouble2x32, packSnorm2x16, packSnorm4x8, packUnorm2x16, packUnorm4x8, unpackDouble2x32,
    unpackSnorm2x16, unpackSnorm4x8, unpackUnorm2x16, unpackUnorm4x8,
};

pub use self::geom::{cross, distance, dot, faceforward, length, normalize, reflect, refract};

pub use self::matrix::{determinant, inverse, matrixCompMult, outerProduct, transpose};

pub use self::vecrel::{
    all, any, equal, greaterThan, greaterThanEqual, lessThan, lessThanEqual, not, notEqual,
};

pub use self::integer::{
    bitCount, bitfieldExtract, bitfieldInsert, bitfieldReverse, findLSB, findMSB, imulExtended,
    uaddCarry, umulExtended, usubBorrow,
};

pub use self::noise::{noise1, noise2, noise3, noise4};

mod common;
mod exp;
mod geom;
mod integer;
mod matrix;
mod noise;
mod pack;
mod trig;
mod vecrel;
