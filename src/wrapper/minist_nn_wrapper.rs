use tch::{Tensor, TchError};
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};
use crate::wrapper::Wrapper;
use crate::obj::predict_result::ProbResult;
use crate::dataframe::Dataframe;


const IMAGE_DIM: i64 = 784;
const HIDDEN_NODES: i64 = 128;
const LABELS: i64 = 10;

impl Dataframe  for Tensor {}

pub struct MnistNNWrapper{
    model : Box<dyn Module>
}

impl MnistNNWrapper {

    fn net(vs: &nn::Path) -> impl Module {
        nn::seq()
            .add(nn::linear(vs / "layer1", IMAGE_DIM, HIDDEN_NODES, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
    }

    fn train(X_train: &Tensor, X_test: &Tensor, y_train: &Tensor, y_test: &Tensor) -> Result<Self, TchError>{

        let vs = nn::VarStore::new(Device::Cpu);
        let net = MnistNNWrapper::net(&vs.root());
        let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
        for epoch in 1..200 {
            let loss = net.forward(&X_train).cross_entropy_for_logits(&y_train);
            opt.backward_step(&loss);
            let test_accuracy = net.forward(&X_test).accuracy_for_logits(&y_test);
            println!(
                "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
                epoch,
                f64::from(&loss),
                100. * f64::from(&test_accuracy),
            );
        }
        Ok(MnistNNWrapper{model:Box::new(net)})
    }


}


impl Wrapper<Tensor, ProbResult> for MnistNNWrapper{


    fn predict(self: &Self, data:& Tensor) -> ProbResult{

        let predict_result = self.model.forward(&data);

        ProbResult{result:predict_result.double_value(&[0;1])}
    }

    fn metrics(self: &Self) {
        
    }
}

#[cfg(test)]
mod mnist_nn_wrapper_tests{
    use crate::wrapper::minist_nn_wrapper::MnistNNWrapper;
    use std::{fs, env};

    #[actix_web::test]
    pub async fn test_training(){

        let m = tch::vision::mnist::load_dir("data").unwrap();
        let X_train = &m.train_images;
        let X_test = &m.test_images;
        let y_train = &m.train_labels;
        let y_test = &m.test_labels;

        let model = MnistNNWrapper::train(X_train, X_test, y_train, y_test).unwrap();

    }
}
