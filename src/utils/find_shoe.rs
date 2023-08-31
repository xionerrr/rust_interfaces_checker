use super::error_res;
use crate::models::Error;

#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

trait Item {
    type K;
    fn f(param: Self::K);
}

impl Item for Shoe {
    type K = Vec<String>;
    fn f(param: Self::K) {
        param.iter().for_each(|item| println!("{:?}", item));
    }
}

pub fn find_shoe(shoes: Vec<Shoe>, shoe_size: u32) -> Result<Option<Shoe>, error_res::ErrorInfo> {
    let res_data: Option<Shoe> = shoes.into_iter().find(|shoe| shoe.size == shoe_size);

    Shoe::f(vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ]);

    if res_data.is_none() {
        let error = Error::error_response(&Error::NotFound);
        return Err(error);
    } else {
        Ok(res_data)
    }
}
