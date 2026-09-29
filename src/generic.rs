#[inline(always)]
pub const fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else{rcl as f32}
}

#[inline(always)]
pub const fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else{-floor(-x+0.5)}
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

#[inline]
pub const fn is_prime(x:u32)->bool{
    let mut sqrt_x:u32=round(sqrt(x as f32))as u32;
    while sqrt_x>2{if sqrt_x==0{return false}sqrt_x-=1}
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

#[inline]
pub const fn arithmetic_mean(x:&[f32])->f32{
    let x_len:usize=x.len();
    if x_len==0{return 0.0}
    let mut result:f32=0.0;
    let mut counter:usize=0;
    while counter<x_len{counter+=1;result+=(x[counter-1]-result)/(counter as f32)}
    return result
}

#[inline]
pub const fn arithmetic_average(x:&[f32])->f32{// Alias for arithmetic_mean
    arithmetic_mean(x)
}