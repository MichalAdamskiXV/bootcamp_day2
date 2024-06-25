#[ic_cdk::query]

//typy danych
//u8 - liczba od 0 do 255
//i8 - liczba od -127 do 128 

fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}
