
#[derive(Debug, Clone)]
pub struct LiveTextString {
    pub text: String
}


mod component;
pub use self::component::*;

mod layout_element;
pub use self::layout_element::*;
