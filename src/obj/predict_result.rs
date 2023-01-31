pub trait PredictResult<T>{
    fn get_result(self:&Self) -> T;
}

pub struct ProbResult{
    pub result: f64
}

impl PredictResult<f64> for ProbResult{
    fn get_result(self:&Self) -> f64{
        self.result
    }
}