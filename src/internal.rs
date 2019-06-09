use super::*;

pub trait Sealed {} // used for sealing traits

impl Sealed for MockBot {}
impl Sealed for Bot {}
