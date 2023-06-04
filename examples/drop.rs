struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!(" => Dropping {}",self.name);
    }    
}
fn main() {
    let _a = Droppable{name : "reaminder"};
    {
        let _b = Droppable{name: "test"};
        {
            let _c = Droppable{name: "c"};
            let _d= Droppable{name: "d"};
            println!("exiting block b");
        }
        println!("just exited block 8");
    }
}