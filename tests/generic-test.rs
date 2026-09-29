use smath::generic::*;

#[test]
fn test_arithmetic_mean(){
    assert_eq!(arithmetic_mean(&[]),0.0);
    assert_eq!(arithmetic_mean(&[3.5]),3.5);

    assert_eq!(arithmetic_mean(&[1.0,2.0,3.0]),2.0);
    assert_eq!(arithmetic_mean(&[-1.0,1.0]),0.0);
    assert_eq!(arithmetic_mean(&[-5.0,1.5,-7.0]),-3.5);
    assert_eq!(arithmetic_mean(&[2.0,100.0,5.0,3.0]),arithmetic_average(&[100.0,2.0,3.0,5.0]))
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(&[]),1);
    assert_eq!(gcd(&[23]),23);
    assert_eq!(gcd(&[0]),1);
    assert_eq!(gcd(&[0,0]),1);
    assert_eq!(gcd(&[0,0,0]),1);

    assert_eq!(gcd(&[9,6]),3);
    assert_eq!(gcd(&[27,81]),27);
    assert_eq!(gcd(&[56,49]),7);
    assert_eq!(gcd(&[0,30]),30);

    assert_eq!(gcd(&[35,100,200,1000,12345]),5);
}

#[test]
fn test_lcm(){
    assert_eq!(lcm(&[]),0);
    assert_eq!(lcm(&[0,0]),0);
    assert_eq!(lcm(&[0,0,0]),0);
    assert_eq!(lcm(&[0,3]),0);
    assert_eq!(lcm(&[0,3,6]),0);

    assert_eq!(lcm(&[1]),1);
    assert_eq!(lcm(&[1,2]),2);
    assert_eq!(lcm(&[2,4]),4);
    assert_eq!(lcm(&[4,5,2]),20);
    assert_eq!(lcm(&[199,199]),199);
}

#[test]
fn test_floor_celling_trunc_round(){
    assert_eq!(round(-0.0),0.0);
    assert_eq!(round(2.0),2.0);
    assert_eq!(round(505.23),505.0);
    assert_eq!(round(27.5),28.0);
    assert_eq!(round(1.6),2.0);
    assert_eq!(round(-3.5),-4.0);
    assert_eq!(round(-7.6545),-8.0);

    assert_eq!(floor(0.0),0.0);
    assert_eq!(floor(2.2),2.0);
    assert_eq!(floor(3.5),3.0);
    assert_eq!(floor(8.8888),8.0);
    assert_eq!(floor(-0.3),-1.0);
    assert_eq!(floor(-19.001),-20.0);
    assert_eq!(floor(-21.777),-22.0);

    assert_eq!(celling(0.0),0.0);
    assert_eq!(celling(1.1),2.0);
    assert_eq!(celling(4.7),5.0);
    assert_eq!(celling(3.0001),4.0);
    assert_eq!(celling(-20.05),-20.0);
    assert_eq!(celling(-30.99),-30.0);

    assert_eq!(floor_digits(2.13,1),2.1);
    assert_eq!(floor_digits(5.777,2),5.77);
    assert_eq!(celling_digits(2.13,1),2.2);
    assert_eq!(celling_digits(5.777,2),5.78);
    assert_eq!(round_digits(2.13,1),2.1);
    assert_eq!(round_digits(5.777,2),5.78);
    assert_eq!(trunc_digits(2.13,1),2.1);
    assert_eq!(trunc_digits(5.777,2),5.77)
}

#[test]
fn test_sqrt(){
    use core::f32::consts::SQRT_2;

    assert!(sqrt(0.0)<0.001&&sqrt(0.0)>-0.001);
    assert!(sqrt(1.0)<1.001&&sqrt(1.0)>0.999);
    assert!(sqrt(4.0)<2.001&&sqrt(4.0)>1.999);

    assert!(sqrt(2.0)<SQRT_2+0.0001&&sqrt(2.0)>SQRT_2-0.0001);
    assert!(sqrt(3.0)<1.7320+0.0001&&sqrt(3.0)>1.7320-0.0001);

    assert!(sqrt(-1.0).is_nan());
    assert!(sqrt(-105.0).is_nan());
}

#[test]
fn test_prime(){
    assert!(is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));

    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(27));
    assert!(!is_prime(123));
    assert!(!is_prime(12345678));
}