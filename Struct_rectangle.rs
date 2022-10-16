#[derive(Debug)]
struct Rectangle{
    height : u32,
    width : u32
}

impl Rectangle{
    fn area(&self) -> u32{ 
        self.width * self.height
    }
    fn print_rec(&self){
        println!("{:#?}",self);
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    //associative block
    fn build_rect(height:u32,width:u32) -> Rectangle{
        Rectangle{
            height:height,
            width:width
        }
    }
    fn sqaure(size:u32)->Rectangle{
        Rectangle{
            height:size,
            width:size
        }
    }
}

 
fn build_rect(height:u32,width:u32) -> Rectangle{
    Rectangle{
        height:height,
        width:width
    }
}


fn main(){
    let rec1=Rectangle::build_rect(50,60);
    let rec2=Rectangle::build_rect(30,40);
    let rec3=Rectangle::build_rect(20,30);
    let rec4=Rectangle::sqaure(40);
    rec4.print_rec();
    // rec1.print_rec();
    // println!("{}",rec1.can_hold(&rec2));
    // println!("{}",rec3.can_hold(&rec2));

    

}