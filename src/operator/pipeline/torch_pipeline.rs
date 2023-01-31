use crate::Operator;

pub struct TorchPipeline{
    schedule : &mut Vec<Operator>,
}

impl TorchPipeline for Operator{

    fn new(self : &Self) -> TorchPipeline{
        self.schedule = Vec::new();
    }

    fn add(self : &Self, operator: Operator){
        self.schedule.add(operator);
    }

    fn run(self: &Self, data: Data){
        for child in self.schedule{
            child.run(data)
        }
    }

}