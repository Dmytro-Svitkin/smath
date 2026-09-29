use smath::trigonometry::*;

#[test]
fn int_trig_values(){
    let angles:[isize;14]=[0,15,30,45,60,75,90,105,120,135,150,165,180,-12342];

    for a in angles{
        println!("▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁");
        println!("█ {}°",a);
        println!("█");

        println!("█ sin({}.0)   = {}",a as f32,sin(a as f32));
        println!("█ sin({})     = {}",a,sin_int(a));
        println!("█");
        assert_eq!(((sin(a as f32)*10.0).round())*0.1,((sin_int(a)*10.0).round())*0.1);

        println!("█ cos({}.0)   = {}",a as f32,cos(a as f32));
        println!("█ cos({})     = {}",a,cos_int(a));
        println!("█");
        assert_eq!(((cos(a as f32)*10.0).round())*0.1,((cos_int(a)*10.0).round())*0.1);

        println!("█ tan({}.0)   = {}",a as f32,tan(a as f32));
        println!("█ tan({})     = {}",a,tan_int(a));
        println!("█");
        assert_eq!(((tan(a as f32)*10.0).round())*0.1,((tan_int(a)*10.0).round())*0.1);

        println!("█ cotan({}.0) = {}",a as f32,cotan(a as f32));
        println!("█ cotan({})   = {}",a,cotan_int(a));
    
        assert_eq!(((cotan(a as f32)*10.0).round())*0.1,((cotan_int(a)*10.0).round())*0.1);
    }
    println!("")
}