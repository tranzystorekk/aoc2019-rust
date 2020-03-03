pub trait IoProvider {
    fn send_input(&mut self) -> i64;

    #[allow(unused_variables)]
    fn get_output(&mut self, value: i64) {}
}

pub struct ValueProvider {
    value: i64
}

impl ValueProvider {
    pub fn new(value: i64) -> Self {
        ValueProvider {
            value
        }
    }
}

impl IoProvider for ValueProvider {
    fn send_input(&mut self) -> i64 {
        self.value
    }
}
