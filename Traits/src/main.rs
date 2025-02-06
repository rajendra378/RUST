trait Tool{
    fn use_tool(&self);//define a rule :: all tool must hacve a"use - tool" function
}

struct Hammer;
struct ScrewDriver;
struct Wrench;

// each tool implements the 'tool' trait and provides its own behavior
impl Tool for Hammer{
    fn use_tool(&self){
        println!("Hammer : Banging nails");
    }
}

impl Tool for ScrewDriver{
    fn use_tool(&self){
        println!("SCrewDriver : Turning Screws");
    }
}

impl Tool for Wrench{
    fn use_tool(&self){
        println!("Wrench : Tighten the Bolts");
    }
}

// now we can write a function that works with any tool.It doesn't care it is a hammerm,
// screwDriver,Wrench it just know the tool can do_a_task


fn perfom<T:Tool>(tool:T){
    tool.use_tool();
}

fn main(){
    let hammer = Hammer;
    let screwDriver = ScrewDriver;
    let wrench = Wrench;

    perfom(hammer);
    perfom(screwDriver);
    perfom(wrench);
}