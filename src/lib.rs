pub struct AverageCollection{
    list: Vec<i64>,
    average: f64
}

impl AverageCollection {
    pub fn add(&mut self,value:i64){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self,)->Option<i64>{
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }

    pub fn average(&self)->f64{
        self.average
    }

    fn update_average(&mut self){
        let total = self.list.iter().sum();
        self.average = total / self.list.len() as f64;
    }

}


pub trait Displaying{
    fn display(&self);
}


pub struct Screen<T: Displaying> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where
        T: Displaying,
{
    pub fn execute(&self) {
        for component in self.components.iter() {
            component.afficher();
        }
    }
}

pub struct Button{
    pub width:u64,
    pub height:u64,
    pub label:String
}

impl Displaying for Button {
    fn display(&self) {
        println!("hey it's a button")
    }
}


pub struct Ticket{
    state:Option<Box<dyn State>>,
    content:String
}

impl Ticket {
    pub fn new()->Ticket{
        Ticket{
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self)-> &str{
        ""
    }

    pub fn ask_verification(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.ask_verification())
        }
    }
}

trait State{
    fn ask_verification(self:Box<Self>)->Box<dyn State>;
}

struct Draft{}

impl State for Draft {
    fn ask_verification(self: Box<Self>) -> Box<dyn State> {
        Box::new(InVerification {})
    }
}

struct InVerification{
    
}

impl State for InVerification {
    fn ask_verification(self: Box<Self>) -> Box<dyn State> {
        self
    }
}