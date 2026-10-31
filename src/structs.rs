pub struct Value<T:Copy>{
    pub(crate)value:Option<T>
}

impl<T:Copy>Value<T>{
    pub const fn from(value:Option<T>)->Self{
        Self{value}
    }

    pub const fn from_unwrapped(unwrapped_value:T)->Self{
        Self{value:Some(unwrapped_value)}
    }

    pub const fn value(&self)->Option<T>{
        self.value
    }

    pub const fn value_unwrapped(&self)->T{
        self.value().unwrap()
    }

    pub const fn is_none(&self)->bool{
        self.value().is_none()
    }

    pub const fn value_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.value_unwrapped()
    }

    pub fn value_unwrapped_or_else<F:FnOnce()->T>(&self,or_else:F)->T{
        self.value().unwrap_or_else(or_else)
    }

    /*
    pub const fn value_unwrapped_or_0(&self)->T{
        self.value_unwrapped_or(type_zero)
    }
    */
}

pub struct Angle<T:Copy>{
    pub(crate)angle:Value<T>
}

impl<T:Copy>Angle<T>{
    pub const fn from_scaled(scaled_angle:Option<T>)->Self{
        Self{angle:Value::from(scaled_angle)}
    }

    pub const fn from_scaled_unwrapped(unwrapped_scaled_angle:T)->Self{
        Self{angle:Value::from_unwrapped(unwrapped_scaled_angle)}
    }

    pub const fn scaled_angle(&self)->Option<T>{
        self.angle.value
    }

    pub const fn scaled_angle_unwrapped(&self)->T{
        self.scaled_angle().unwrap()
    }

    pub const fn is_none(&self)->bool{
        self.scaled_angle().is_none()
    }

    pub const fn scaled_angle_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.scaled_angle_unwrapped()
    }

    pub fn scaled_angle_unwrapped_or_else<F:FnOnce()->T>(&self,or_else:F)->T{
        self.scaled_angle().unwrap_or_else(or_else)
    }
}