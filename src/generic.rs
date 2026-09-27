/// Simplified square root.
#[inline]
pub const fn sqrt(x:f32)->f32{
    if x<0.0{return f32::NAN;}if x==0.0||x==f32::INFINITY{return x;}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));let rcl:f32=x*rcl;0.5*(rcl+x/rcl)
}

/// Simplified inverse square root.
#[inline]
pub const fn isqrt(x:f32)->f32{
    if x<0.0{return f32::NAN;}if x==0.0||x==f32::INFINITY{return 0.0;}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));rcl
}

#[inline]
pub const fn abs(x:f32)->f32{
    if x<0.0{return-x}
    x
}

#[inline]
pub const fn factorial(x:u32)->u32{
    let mut result:u32=1;
    let mut x:u32=x;
    while x>1{result*=x;x-=1}
    result
}