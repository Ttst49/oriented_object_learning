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

