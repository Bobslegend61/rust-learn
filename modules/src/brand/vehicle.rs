#[derive(Debug)]
pub enum Toyota {
    Camry,
    Corolla,
    Highlander,
}

#[derive(Debug)]
pub enum CarBrands {
    Toyota(Toyota),
    Mercedes,
    Kia,
    Honda,
    Tesla
}

#[derive(Debug)]
pub enum Vehicle {
    Car(CarBrands),
    Bus
}
