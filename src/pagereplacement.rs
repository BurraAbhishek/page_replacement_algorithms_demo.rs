pub struct PageReplacement {
    pub page_reference: Vec<String>,
    pub frame: Vec<Vec<String>>,
    pub status: Vec<String>,
    pub framecount: u8,
}

impl Default for PageReplacement {
    fn default() -> PageReplacement {
        PageReplacement {
            page_reference: Vec::<String>::new(),
            frame: Vec::<Vec<String>>::new(),
            status: Vec::<String>::new(),
            framecount: 0,
        }
    }
}

impl PageReplacement {
    pub fn add_page_string(mut self, a: Vec<String>, n: u8) -> PageReplacement {
        self.page_reference = a;
        self.framecount = n;
        return self;
    }
}
