use crate::obj::data::Data;
use crate::obj::score::Score;

pub fn chatgpt_predict(data: Data) -> Score{
    Score{score: Some(0.5)}
}