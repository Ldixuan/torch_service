
pub trait  Wrapper<D, R>{

    fn predict(self: &Self, data: &D) -> R;

    fn metrics(self: &Self);
}

pub mod minist_nn_wrapper;