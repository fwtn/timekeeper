//use lib::Alert;
use crate::alert::Alert;
pub struct PresetTimer{
  second: i32,
  name: String,
}
impl PresetTimer{
  pub fn SetTimer(&self){
  }
}
impl Alert for PresetTimer{
    fn check(&self) -> bool{
        return true;
    }
    fn alert(&self){}
}