pub struct Value<T:Copy>{
    pub(crate)value:Option<T>
}

impl<T:Copy>Value<T>{
    pub const fn as_raw_value_unwrapped(&self)->T{
        self.value.unwrap()
    }

    pub const fn as_raw_value(&self)->Option<T>{
        self.value
    }

    pub const fn as_raw_value_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.as_raw_value_unwrapped()
    }

    pub const fn from_unwrapped(value:T)->Self{
        Self{value:Some(value)}
    }

    pub const fn from(raw_value:Option<T>)->Self{
        Self{value:raw_value}
    }

    pub const fn is_none(&self)->bool{
        self.value.is_none()
    }
}

pub struct Angle<T:Copy>{
    pub(crate)angle:Value<T>
}

impl<T:Copy>Angle<T>{
    pub const unsafe fn as_raw_value_unwrapped(&self)->T{
        self.angle.as_raw_value_unwrapped()
    }

    pub const fn as_raw_value(&self)->Option<T>{
        self.angle.as_raw_value()
    }

    pub const fn as_raw_value_unwrapped_or(&self,default:T)->T{
        self.angle.as_raw_value_unwrapped_or(default)
    }

    pub const fn raw_from_unwrapped(raw_angle_value:T)->Self{
        Self{angle:Value::from_unwrapped(raw_angle_value)}
    }

    pub const fn raw_from(raw_angle_value:Option<T>)->Self{
        Self{angle:Value::from(raw_angle_value)}
    }

    pub const fn is_none(&self)->bool{
        self.angle.is_none()
    }
}