use super::Client;

impl Client {
    /// Define a new drug
    pub fn define_drug(self, name: &String) {
        println!("New drug {name} has been defined.");
    }
}
