use smath::trigonometry::*;

#[test]
fn test_degrees(){
    assert!((sin(30.0)-0.5000000).abs()<0.001);
    assert!((cos(60.0)-0.5000000).abs()<0.001);
    assert!((tan(45.0)-1.0000000).abs()<0.001);
    assert!((cotan(45.0)-1.0000000).abs()<0.001);
}

#[test]
fn test_radians(){
    assert!((sinr(30.0/180.0)-0.5000000).abs()<0.001);
    assert!((cosr(60.0/180.0)-0.5000000).abs()<0.001);
    assert!((tanr(0.25)-1.0000000).abs()<0.001);
    assert!((cotanr(0.25)-1.0000000).abs()<0.001);
}

#[test]
fn test_arcs(){
    assert_eq!((tan(atan(0.5))*1000.0).round()/1000.0,(tanr(atanr(0.5))*1000.0).round()/1000.0);
    assert_eq!((sin(asin(0.2))*1000.0).round()/1000.0,(sinr(asinr(0.2))*1000.0).round()/1000.0);
    assert_eq!((sin(asin(-0.6))*1000.0).round()/1000.0,(cos(acos(-0.6))*1000.0).round()/1000.0);
    assert_eq!((tanr(atanr(3.3))*1000.0).round()/1000.0,(cotanr(acotanr(3.3))*1000.0).round()/1000.0);
}