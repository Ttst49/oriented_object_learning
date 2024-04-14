pub struct AverageCollection{
    list: Vec<i64>,
    average: f64
}

impl AverageCollection {
    pub fn add(&mut self,value:i64){
        self.list.push(value);
        self.updateAverage();
    }

    pub fn remove(&mut self,)->Option<i64>{
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.updateAverage();
                Some(value)
            }
        }
    }

    pub fn average(&self)->f64{
        self.average
    }

    pub fn updateAverage(&mut self){
        let total = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}