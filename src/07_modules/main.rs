mod front_of_house;

fn main() {
    front_of_house::hellowers::greeters::hello();
    front_of_house::hellowers::not_included::unauthorized_hello();
}