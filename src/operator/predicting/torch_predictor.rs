pub struct TorchPredictor{
    model : TorchModel
}

impl TorchModel for Operator{

    fn new(self: &Self, model: TorchModel){
        self.model = model;
    }


}