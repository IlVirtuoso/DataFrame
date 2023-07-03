use Columns::IColumn;

mod Columns;

pub struct DataFrame{
      columns: Vec<Box<dyn IColumn>>

}

impl DataFrame{
      pub fn new(){
            
      }
}