#[path = "fifo.rs"]
mod fifo_page;
#[path = "input.rs"]
mod input_rs;
#[path = "page.rs"]
mod page_rs;
use fifo_page::page_replacement::PageReplacement;

fn main() {
    let mut demopage = page_rs::Page {
        ..Default::default()
    };
    demopage = demopage.generate();
    println!("Number of frames: ?");
    let frames: u8 = input_rs::input_u8();
    let pagestring = demopage.get_page_string();
    let mut pagereplacement = PageReplacement {
        ..Default::default()
    };
    pagereplacement = pagereplacement.add_page_string(pagestring, frames);
    let fifopages = fifo_page::fifo(pagereplacement);
    for i in 0..fifopages.status.len() {
        for j in 0..fifopages.frame[i].len() {
            println!("{}", fifopages.frame[i][j]);
        }
        println!("{}", fifopages.status[i]);
    }
    println!("Completed.");
    println!("At the moment, only FIFO replacement is supported. ");
    println!("Support for more algorithms coming soon...");
}
