pub trait Future {
    type Output;

    fn poll(&mut self) -> Option<Self::Output>;
}
