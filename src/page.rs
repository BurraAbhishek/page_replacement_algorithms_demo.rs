#[path = "input.rs"] mod input;
use input as other_input;
//use other_input::vec_strings;

pub struct Page {
    pub page_string: Vec<String>,
}

impl Default for Page {
    fn default() -> Page {
        let sequence = "";
        let new_page_string = crate::vec_strings!(sequence.chars().collect::<String>());
        Page {page_string: new_page_string}
    }
}

impl Page {
    fn sequentially(mut self) {
        let sequence = other_input::input();
        let collected = sequence.chars();
        for s in collected {
            let newstr = String::from(s);
            self.page_string.push(newstr);
        }
    }
    fn commaseparated(mut self) {
        let sequence = other_input::input();
        let cleansequence = sequence.split(",");
        let collected: Vec<&str> = cleansequence.collect();
        for s in collected {
            let newstr = String::from(s);
            let trimmed = newstr.trim();
            self.page_string.push(String::from(trimmed));
        }
    }
    fn linebyline(mut self) {
        loop {
            let inserted: String = other_input::input();
            println!("{}", inserted.len());
            if inserted.len() == 1 {break;}
            self.page_string.push(inserted);
        }
    }
    pub fn generate(self) {
        println!("Choose any of the below options: ");
        println!("0: Enter the page reference string sequentially (Example: 102341)");
        println!("1: Enter the comma-separated page reference string (Example: 1, 0, 2, 3, 4, 1)");
        println!("2: Enter the page reference string one after the other");
        let choice = other_input::input_u8();
        match choice {
            0 => self.sequentially(),
            1 => self.commaseparated(),
            2 => self.linebyline(),
            3..=255 => panic!("Invalid choice")
        }
    }
}
