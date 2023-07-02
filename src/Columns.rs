use std::{any::{TypeId, Any}, ops::{Add, Div, Index}, iter::Sum, fmt::format};


//region struct



pub trait IColumn{
    fn get_type(&self)-> &String;
    fn get_name(&self) -> &String;
}



pub struct Column<T>{
      data: Vec<T>,
      typeinfo : String,
      name: String
}
//endregion

impl<T> Column<T> where T: Clone{
      pub fn new(name : String) -> Self{
            Column { data: Vec::new(), typeinfo: String::from(std::any::type_name::<T>(), ), name}
      }

      pub fn from(name : String, itr : impl Iterator<Item = T>) -> Self{
        let mut data : Vec<T> = Vec::new();
        for i in itr{
            data.push(i);
        }
        Column { data, typeinfo: String::from(std::any::type_name::<T>()), name }
      }

      pub fn fromvec(name:String, vec:Vec<T>) -> Self{
        Column{data:vec,typeinfo: String::from(std::any::type_name::<T>()),name }
      }
}
impl<T> IColumn for Column<T> where T: Add + Div{
    fn get_type(&self)-> &String {
        &self.typeinfo
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl<T> PartialEq for Column<T> {
    fn eq(&self, other: &Self) -> bool {
      self.typeinfo == other.typeinfo && self.name == other.name
    }
}


impl<T> Eq for Column<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> Add for Column<T> where T: Add<Output = T> + Clone{
    type Output = Column<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.data.len() == rhs.data.len());
        let name = format!("{} + {}",self.name,rhs.name);
        let mut data = Vec::<T>::new();
        for i in 0..self.data.len(){
            data.push(self.data[i].clone() + rhs.data[i].clone());
        }
        Column::fromvec(name, data)
    }
}

impl<T> Div for Column<T> where T : Div{
    type Output = T;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }

}

impl<T> Index<usize> for Column<T>{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}






#[cfg(test)]
mod tests {
    use cute::c;

    use super::*;

    #[test]
    fn test_type() {
        let col = Column::<i32>::new("test".to_string());
        println!("name {}",col.name);
        println!("type {}", col.typeinfo);
        let colstring = Column::<String>::new("teststring".to_string());
        println!("typestring {}",colstring.typeinfo);
    }

    #[test]
    fn test_assign(){
        let vec = c![x, for x in 1..99];
        let col  = Column::from("test".to_string(),vec.iter());
        assert_eq!(1,col[0].to_owned());
    }

    #[test]
    fn test_sum(){
        let v1 = Column::fromvec("h1".to_string(), c![x ,for x in 1..99]);
        let v2 = Column::fromvec("h1".to_string(), c![x ,for x in 1..99]);
        let sum = v1 + v2;
        assert_eq!(4,sum[1]);
    }

    #[test]
    fn test_inv(){
        let v1: &mut dyn IColumn = &mut Column::fromvec("h1".to_string(), c![x ,for x in 1..99]);
        assert_eq!("h1",v1.get_name());
    }
}