pub struct AverageCollection{
    list: Vec<i64>,
    average: f64
}

impl AverageCollection {
    pub fn add(&mut self,value:i64){
        self.list.push(value);
        self.updateAverage();
    }

}