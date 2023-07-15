use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MIL_TO_ENG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("alpha", "A");
        map.insert("bravo", "B");
        map.insert("charlie", "C");
        map.insert("delta", "D");
        map.insert("echo", "E");
        map.insert("foxtrot", "F");
        map.insert("golf", "G");
        map.insert("hotel", "H");
        map.insert("india", "I");
        map.insert("juliett", "J");
        map.insert("kilo", "K");
        map.insert("lima", "L");
        map.insert("mike", "M");
        map.insert("november", "N");
        map.insert("oscar", "O");
        map.insert("papa", "P");
        map.insert("quebec", "Q");
        map.insert("romeo", "R");
        map.insert("sierra", "S");
        map.insert("tango", "T");
        map.insert("uniform", "U");
        map.insert("victor", "V");
        map.insert("whiskey", "W");
        map.insert("xray", "X");
        map.insert("x-ray", "X");
        map.insert("yankee", "Y");
        map.insert("zulu", "Z");
        map
    };
}

lazy_static! {
    pub static ref ENG_TO_MIL: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("a", "Alpha");
        map.insert("b", "Bravo");
        map.insert("c", "Charlie");
        map.insert("d", "Delta");
        map.insert("e", "Echo");
        map.insert("f", "Foxtrot");
        map.insert("g", "Golf");
        map.insert("h", "Hotel");
        map.insert("i", "India");
        map.insert("j", "Juliett");
        map.insert("k", "Kilo");
        map.insert("l", "Lima");
        map.insert("m", "Mike");
        map.insert("n", "November");
        map.insert("o", "Oscar");
        map.insert("p", "Papa");
        map.insert("q", "Quebec");
        map.insert("r", "Romeo");
        map.insert("s", "Sierra");
        map.insert("t", "Tango");
        map.insert("u", "Uniform");
        map.insert("v", "Victor");
        map.insert("w", "Whiskey");
        map.insert("x", "X-ray");
        map.insert("y", "Yankee");
        map.insert("z", "Zulu");
        map
    };
}
