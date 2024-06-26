use std::cell::{RefCell};

//sekcja danych
thread_local! {
    // static to pole w pamięci, które będzie żyło cały czas
    // RefCell - można korzystać z danych ze zmiennej statycznej
    // Vec - wektor stringów
    // WPISY - lista stringów 
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
// ic_cdk::query - metoda odczytu danych dot. funkjcjii poniżej 
// typy danych
// u8 - liczba od 0 do 255
// i8 - liczba od -127 do 128 
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update] //do odczytu danych potrzebujemy wszystkie hosty - komputery 
fn dodaj_wpis(wpis: String) {
    //mamy zmienną wpis
    //wpychamy ją do wszystkich wpisów (do tego wektora)
    //bierzemy WPISY - with przyjmuje funkcje (wpisy to zmienna wypakowana) - with -odwołumey się (dzwonimy) do WPISY
    //wpisy.borrow_mut() - porzyczamy zmienną (jako edytowalną _mut()) i zwracamy .push(wpis), w taki sposób by nie zgubić poprzednich wartości
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().push(wpis)
    });
}

#[ic_cdk::query] //zwracamy dane
fn odczytaj_wpisy() -> Vec<String> {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        //.borrow() - porzyczamy wpisy
        //.clone() - kopiujemy te wpisy - otwieramy pudełko, kopujemy dane i zamykamy tworząc kopie.
        wpisy.borrow().clone() //return - bo nie daliśmy średnika xd
    })
}

//u32 - wspierane przez wektor od 0 do 2^32 - 1;
//usize - liczba uzależniona od komputera 
#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().remove(id_wpisu)
    });
}

#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize, nowy_wpis: String) {
    WPISY.with(|wpisy| {
        let mut binding = wpisy.borrow_mut();
        let wpis = binding.get_mut(id_wpisu);
        let stary_wpis = wpis.unwrap();
        *stary_wpis = nowy_wpis;
    });
}