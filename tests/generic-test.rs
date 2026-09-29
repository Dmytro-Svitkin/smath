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
fn test_round_floor(){
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