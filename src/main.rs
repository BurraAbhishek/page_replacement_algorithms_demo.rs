#[path = "fifo.rs"]
mod fifo_page;
#[path = "input.rs"]
mod input_rs;
#[path = "page.rs"]
mod page_rs;
#[path = "pagereplacement.rs"]
mod page_replacement;

fn main() {
    let mut demopage = page_rs::Page {
        ..Default::default()
    };
    demopage = demopage.generate();
    println!("Number of frames: ?");
    let frames: u8 = input_rs::input_u8();
    let pagestring = demopage.get_page_string();
    let mut pagereplacement = page_replacement::PageReplacement {
        ..Default::default()
    };
    pagereplacement = pagereplacement.add_page_string(pagestring, frames);
    let processedpages = fifo(pagereplacement);
    for i in 0..processedpages.status.len() {
        for j in 0..processedpages.frame[i].len() {
            println!("{}", processedpages.frame[i][j]);
        }
        println!("{}", processedpages.status[i]);
    }
    println!("Completed.");
    println!("At the moment, only FIFO replacement is supported. ");
    println!("Support for more algorithms coming soon...");
}

fn fifo(pagereplacement: page_replacement::PageReplacement) -> page_replacement::PageReplacement {
    let mut prefifopages = fifo_page::page_replacement::PageReplacement {
        page_reference: Vec::<String>::new(),
        frame: pagereplacement.frame,
        status: pagereplacement.status,
        framecount: 0,
    };
    prefifopages = prefifopages.add_page_string(pagereplacement.page_reference, pagereplacement.framecount);
    let fifopages = fifo_page::fifo(prefifopages);
    let processedpages = page_replacement::PageReplacement {
        page_reference: fifopages.page_reference,
        frame: fifopages.frame,
        status: fifopages.status,
        framecount: fifopages.framecount,
    };
    return processedpages; 
}
