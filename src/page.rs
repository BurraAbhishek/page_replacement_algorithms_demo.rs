#[path = "input.rs"]
mod input;
use input as other_input;

pub struct Page {
    pub page_string: Vec<String>,
}

impl Default for Page {
    fn default() -> Page {
        Page {
            page_string: Vec::<String>::new(),
        }
    }
}

impl Page {
    fn sequentially(mut self) -> Page {
        println!("Enter the page reference string. Separation will be done character-wise");
        let sequence = other_input::input();
        let collected = sequence.chars();
        for s in collected {
            let newstr = String::from(s);
            self.page_string.push(newstr);
        }
        return self;
    }
    fn commaseparated(mut self) -> Page {
        println!("Enter the page reference string. Separation will be done by commas");
        let sequence = other_input::input();
        let cleansequence = sequence.split(",");
        let collected: Vec<&str> = cleansequence.collect();
        for s in collected {
            let newstr = String::from(s);
            let trimmed = newstr.trim();
            self.page_string.push(String::from(trimmed));
        }
        return self;
    }
    fn linebyline(mut self) -> Page {
        println!("Enter the page reference string line-by-line. To stop, just use Enter.");
        loop {
            let inserted: String = other_input::input();
            if inserted.len() == 1 {
                break;
            }
            self.page_string.push(inserted);
        }
        return self;
    }
    pub fn generate(self) -> Page {
        println!("Choose any of the below options: ");
        println!("0: Enter the page reference string sequentially (Example: 102341)");
        println!("1: Enter the comma-separated page reference string (Example: 1, 0, 2, 3, 4, 1)");
        println!("2: Enter the page reference string one after the other");
        let choice = other_input::input_u8();
        match choice {
            0 => self.sequentially(),
            1 => self.commaseparated(),
            2 => self.linebyline(),
            3..=255 => panic!("Invalid choice"),
        }
    }
    pub fn get_page_string(self) -> Vec<String> {
        return self.page_string;
    }
    pub fn _display(self) {
        for i in self.page_string {
            println!("{}", i);
        }
    }
}
