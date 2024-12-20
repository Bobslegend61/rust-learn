use crate::brand::vehicle;

pub mod brand;


fn main() {
    let car = vehicle::Vehicle::Car(vehicle::CarBrands::Toyota(vehicle::Toyota::Corolla));

    match car {
        vehicle::Vehicle::Bus => {
            println!("This is a {:?}", car);
        },
        vehicle::Vehicle::Car(brand) => {
            println!("This is a {:?}", brand);

            if let vehicle::CarBrands::Toyota(model) = brand  {
                println!("Toyota: {:?}", model);
            }else {
                println!("Others")
            }
        }
    }
}
