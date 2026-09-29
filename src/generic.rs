#[inline(always)]
const fn pow10(exponent:usize)->u32{
    let mut exponent:usize=exponent;
    let mut result:u32=1;
    while exponent>0{result*=10;exponent-=1}
    result
}

/// Euclidean floor function.
///
/// Returns the greatest whole number equal or less than the given number.
#[inline(always)]
pub const fn floor(x:f32)->f32{
    let x_trunc:f32=trunc(x);
    if x<x_trunc{x_trunc-1.0}else{x_trunc}
}

/// Euclidean celling function.
///
/// Returns the least whole number equal or less than the given number.
#[inline(always)]
pub const fn celling(x:f32)->f32{
    let rcl:f32=x as i32 as f32;
    if x>rcl{rcl+1.0}else{rcl}
}

///Trunc function.
///
/// Returns the whole part of the given number.
#[inline(always)]
pub const fn trunc(x:f32)->f32{
    x as i32 as f32
}

#[inline(always)]
pub const fn trunc_digits(x:f32,digits:usize)->f32{
    trunc(x*pow10(digits)as f32)
}

/// Round function.
/// 
/// Returns the *closest* *whole* number.
#[inline(always)]
pub const fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else{celling(x-0.5)}
}

#[inline(always)]
pub const fn round_digits(x:f32,digits:usize)->f32{
    round(x*pow10(digits)as f32)
}

/// Simplified square root.
#[inline]
pub const fn sqrt(x:f32)->f32{
    if x<0.0{return f32::NAN}
    if x==0.0||x==f32::INFINITY{return x}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));let rcl:f32=x*rcl;0.5*(rcl+x/rcl)
}

/// Simplified inverse square root.
#[inline]
pub const fn isqrt(x:f32)->f32{
    if x<0.0{return f32::NAN}
    if x==0.0||x==f32::INFINITY{return 0.0}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));rcl
}

/// Absolute value.
/// 
/// Returns a positive number.
#[inline]
pub const fn abs(x:f32)->f32{
    if x<0.0{return-x}
    x
}

/// Factorial.
#[inline]
pub const fn factorial(x:u32)->u32{
    let mut result:u32=1;
    let mut x:u32=x;
    while x>1{result*=x;x-=1}
    result
}

/// Checks if the given number is prime.
/// 
/// Returns `true` if prime.
/// 
/// Returns `false` if non-prime.
#[inline]
pub const fn is_prime(x:u32)->bool{
    if x<2{return false}
    if x==2||x==3{return true}
    if x%2==0||x%3==0{return false}

    let mut d:u32=5;
    
    while d*d<=x{// 6k+-1 rule applied.
        if x%d==0||x%(d+2)==0{return false}
        d+=6;
    }

    true
}

#[inline(always)]
const fn gcd_2(mut a:u32,mut b:u32)->u32{
    while b!=0{
        let rcl:u32=b;
        b=a%b;
        a=rcl;
    }

    if a==0{return 1}
    a
}

/// Greatest common divider.
#[inline]
pub const fn gcd(x:&[u32])->u32{
    let x_len:usize=x.len();

    if x_len==0{return 1}
    else if x_len==1{if x[0]==0{return 1}return x[0]}
    else if x_len==2{return gcd_2(x[0],x[1])}
    
    let mut result:u32=0;
    let mut counter:usize=0;

    while counter<x_len{
        result=gcd_2(result,x[counter]);
        if result==1{return 1}
        counter+=1
    }

    result
}

#[inline(always)]
const fn lcm_2(a:u32,b:u32)->u32{
    if a==0||b==0{return 0}
    (a/gcd_2(a,b))*b
}


/// Least common Multiple.
#[inline]
pub const fn lcm(x:&[u32])->u32{
    if x.is_empty(){return 0}

    let mut result:u32=x[0];
    let mut counter:usize=1;

    while counter<x.len() {
        if result==0{return 0}
        result=lcm_2(result,x[counter]);
        counter+=1;
    }

    result
}

/// Arithmetic mean.
#[inline]
pub const fn arithmetic_mean(x:&[f32])->f32{
    let x_len:usize=x.len();
    if x_len==0{return 0.0}
    let mut result:f32=0.0;
    let mut counter:usize=0;
    while counter<x_len{counter+=1;result+=(x[counter-1]-result)/(counter as f32)}
    return result
}

/// Arithmetic average.
/// 
/// <sup>It is an alias for arithmetic mean.</sup>
#[inline]
pub const fn arithmetic_average(x:&[f32])->f32{// Alias for arithmetic_mean
    arithmetic_mean(x)
}