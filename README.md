# smath

`smath` (Simple Math, Simplified Math or Small Math) is a lightweight, highly optimized Rust mathematical library designed for performance-critical applications. It provides fast, low-overhead implementations of essential mathematical operations with a primary focus on minimizing instruction counts and avoiding expensive operations.

`smath` features *zero* dependencies and is suitable for `no_std` environments.

## Modules

### Generic Mathematics (`smath::generic`)
| **Name** | **Function** | **Input** | **Output** |
| --- | --- | --- | --- |
| Round | `round` | `f32` | `f32` |
| Round leaving decimal digits | `round_digits` | `f32` | `f32` |
| Trunc | `trunc` | `f32` | `f32` |
| Trunc leaving decimal digits | `trunc_digits` | `f32` | `f32` |
| Euclidean Floor | `floor`, | `f32` | `f32` |
| Euclidean Floor leaving decimal digits| `floor_digits`, | `f32` | `f32` |
| Euclidian Celling | `celling` | `f32` | `f32` |
| Euclidian Celling leaving decimal digits | `celling_digits` | `f32` | `f32` |
| Absolute Value | `abs` | `f32` | `f32` |
| Factorial | `factorial` | `u32` | `u32` |
| Prime Number Check | `is_prime` | `u32` | `bool` |
| Square Root | `sqrt` | `f32` | `f32` |
| Inverted Square Root | `isqrt` | `f32` | `f32` |
| Greatest Common Divider | `gcd` | `&[u32]` | `u32` |
| Least Common Multiple | `lcm` | `&[u32]` | `u32` |
| Arithmetic Mean or Arithmetic Average | `arithmetic_mean`/`arithmetic_average` | `&[f32]` | `f32` |

```rust
// Example Usage
const a: [u32; 3] = [round(12.5) as u32, 20, 6]; // [13, 20, 6].
let com_mul = lcm(&a); // 720 is the least common multiple here.
const root = floor_digit(sqrt(com_mul as f32),2); // 27.92.
if !is_prime(684){ // Returns true, baceuse 684 is not a prime number.
    println!("{}",gcd(&[6, 8, 4])) // Prints 2.
}
```

### Trigonometry (`smath::trigonometry`)
To input an angle in degrees, enter the numeric value - for example, `45.0`.  

| **Name** | **Function** | **Input** | **Output** |
| --- | --- | --- | --- |
| Sinus | `sin` | Degrees `f32` | `f32` |
| Cosinus | `cos` | Degrees `f32` | `f32` |
| Tangens | `tan` | Degrees `f32` | `f32` |
| Cotangens | `cotan` | Degrees `f32` | `f32` |
| Arc-Sinus | `asin` | `f32` | Degrees `f32` |
| Arc-Cosinus | `acos` | `f32` | Degrees `f32` |
| Arc-Tangens | `atan` | `f32` | Degrees `f32` |
| Arc-Cotangens | `acotan` | `f32` | Degrees `f32` |

To input an angle in degrees using `isize`, enter the numeric value - for example, `45` (for 45°).
<sub>`isize` inputs are faster and more precise than `f32` values because they use lookup tables.</sub>

| **Name** | **Function** | **Input** | **Output** |
| --- | --- | --- | --- |
| Sinus | `sin_int` | Degrees `isize` | `f32` |
| Cosinus | `cos_int` | Degrees `isize` | `f32` |
| Tangens | `tan_int` | Degrees `isize` | `f32` |
| Cotangens | `cotan_int` | Degrees `isize` | `f32` |

To input an angle in radians, enter the numeric value - for example, `2.0` (for 2π radians).  
There is no need to multiply the input by π; the functions interpret the value as radian.

| **Name** | **Function** | **Input** | **Output** |
| --- | --- | --- | --- |
| Sinus | `sinr` | Radians `f32` | `f32` |
| Cosinus | `cosr` | Radians `f32` | `f32` |
| Tangens | `tanr` | Radians `f32` | `f32` |
| Cotangens | `cotanr` | Radians `f32` | `f32` |
| Arc-Sinus | `asinr` | `f32` | Radians `f32` |
| Arc-Cosinus | `acosr` | `f32` | Radians `f32` |
| Arc-Tangens | `atanr` | `f32` | Radians `f32` |
| Arc-Cotangens | `acotanr` | `f32` | Radians `f32` |

```rust
// Example Usage

const slope = tan(12);       // Slope from a 12° angle (isize is const‑friendly)
let angle_rad = atanr(0.54); // Inverse tangent in radians, returns angle as f32
let sinus_deg = sin(67.5);   // Sinus of a 67.5° angle
```

### Constants (`smath::constant`)
| **Name** | **Category** | **Rust Type** | **Description** |
| --- | --- | --- | --- |
| `SIN` | Trigonometric | `[f32; 91]` | Sinus values for 0°-90° |
| `COS` | Trigonometric | `[f32; 91]` | Cosinus values for 0°-90° |
| `TAN`/`TG` | Trigonometric | `[f32; 91]` | Tangens values for 0°-90° |
| `COTAN`/`CTG` | Trigonometric | `[f32; 91]` | Cotangens values for 0°-90° |
| `PI` | Mathematical | `f32` | π (`3.14`) |
| `E` | Mathematical | `f32` | Euler’s number (`2.72`) |
| `PI_200` | Mathematical | `f64` | High‑precision π (200 digits) |
| `E_200` | Mathematical | `f64` | High‑precision Euler’s number (200 digits) |

### Vectors (`smath::vector`)
2D, 3D, and 4D vector types (`Vec2`, `Vec3`, `Vec4`) with a wide range of mathematical and geometric operations.

| **Category** | **Methods** | **Description** |
| --- | --- | --- |
| Creation & Setup | `new`, `zero`, `one`, `set`, `shift` | Create or modify vector instances |
| Math Operations | `dot`, `cross`, `length`, `sq_length`, `distance`, `sq_distance` | Core vector math and magnitude calculations |
| Normalization | `normalize`, `normalized` | Scale vector to unit length |
| Angles & Rotation | `angle_deg`, `angle_rad`, `rotate`, `rotate_deg`, `rotate_rad` | Compute or apply rotations and angles |
| Interpolation & Clamping | `lerp`, `clamp`, `abs`, `midpoint` | Smooth transitions and value constraints |
| Projection & Reflection | `project`, `reject`, `reflect` | Vector projection and reflection utilities |
| 3D Vector Flattening | `flat`, `sp_flat`, `persp` | Convert or project 3D vectors into 2D space using flat, spherical, or perspective flattening |
| Operators | `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=` | Component‑wise arithmetic via Rust operator overloading |


```rust
// Example Usage

let mut pos = Vec3::zero(); // 3D vector "pos" is equal to {0.0, 0.0, 0.0}
pos.set(10.0, 5.0, 0.0); // "pos" is set to {10.0, 5.0, 0.0}
pos.shift(1.0, 0.0, 0.0); // "pos" is shifted by {1.0, 0.0, 0.0} and is equal to {11.0, 5.0, 0.0}

let velocity = Vec3::new(0.0, 2.0, 1.0); // 3D vector "velocity" is equal to {0.0, 2.0, 1.0}
let next_frame = pos + (velocity * 2.0); // next_frame is equal to {11.0, 9.0, 2.0}
let flat_vel = velocity.flat(); // velocity is flattened to {0.0, 2.0}
```

## Martices (`smath::matrix`)
Matrix types (`Mat3`, `Mat4`, and generic `Mat<ROW, COL>`) providing const‑generic, *column‑major* storage with full arithmetic and transformation support.

| **Category** | **Methods** | **Description** |
| --- | --- | --- |
| Creation & Setup | `new`, `zero`, `fill`, `identity`, `set`, `set_row`, `set_col`, `set_all`, `shift`, `shift_row`, `shift_col`, `shift_all` | Create or modify matrix instances |
| Access | `get`, `get_row`, `get_col` | Retrieve elements, rows, and columns |
| Transposition | `transpose`, `transposed` | Return or apply matrix transpose |
| Trace | `trace` | Sum of diagonal elements |
| Transformations (4×4) | `translation`, `scale`, `rotate_x`, `rotate_y` | Build common 3D transform matrices |
| Projection | `persp` | Perspective projection |
| View Matrix | `view` | Camera look‑at transformation |
| Operators | `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=` | Component‑wise arithmetic via Rust operator overloading |

```rust
// Example Usage

let mut m = Mat4::identity(); // Create a 4×4 identity matrix
let t = Mat4::translation(1.0, 2.0, 3.0); // Apply translation
let combined = m * t; // Combine transforms
let v = Vec4::new(1.0, 0.0, 0.0, 1.0); // Multiply by a vector
let transformed = combined * v;
let trace_val = combined.trace(); // Compute trace
```
## Installation

Add `smath` to your `Cargo.toml` dependencies:

```toml
[dependencies]
smath = "0.5.0"
```

<sub>This project is licensed under the BSD 3-Clause License.</sub>