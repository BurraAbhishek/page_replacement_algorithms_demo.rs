#[path = "input.rs"]
mod input_rs;
#[path = "pagereplacement.rs"]
mod page_replacement;
#[path = "page.rs"]
mod page_rs;
// All page replacement algorithms imports
#[path = "fifo.rs"]
mod fifo_page;
#[path = "lru.rs"]
mod lru_page;
#[path = "optimal.rs"]
mod optimal_page;

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
    let processedpages = choose_algo(pagereplacement);
    for i in 0..processedpages.status.len() {
        for j in 0..processedpages.frame[i].len() {
            println!("{}", processedpages.frame[i][j]);
        }
        println!("{}", processedpages.status[i]);
    }
    println!("Completed.");
}

// Choosing a page replacement algorithm

fn choose_algo(p: page_replacement::PageReplacement) -> page_replacement::PageReplacement {
    println!("Choose a page replacement algorithm");
    println!("0: First-In-First-Out");
    println!("1: Least Recently Used");
    println!("2: Optimal");
    let choice = input_rs::input_u8();
    match choice {
        0 => fifo(p),
        1 => lru(p),
        2 => optimal(p),
        3..=255 => panic!("Invalid choice"),
    }
}

// Add all these page replacement algorithm setup here.

fn fifo(pagereplacement: page_replacement::PageReplacement) -> page_replacement::PageReplacement {
    let mut prefifopages = fifo_page::page_replacement::PageReplacement {
        page_reference: Vec::<String>::new(),
        frame: pagereplacement.frame,
        status: pagereplacement.status,
        framecount: 0,
    };
    prefifopages =
        prefifopages.add_page_string(pagereplacement.page_reference, pagereplacement.framecount);
    let fifopages = fifo_page::fifo(prefifopages);
    let processedpages = page_replacement::PageReplacement {
        page_reference: fifopages.page_reference,
        frame: fifopages.frame,
        status: fifopages.status,
        framecount: fifopages.framecount,
    };
    return processedpages;
}

fn optimal(
    pagereplacement: page_replacement::PageReplacement,
) -> page_replacement::PageReplacement {
    let mut preoptimalpages = optimal_page::page_replacement::PageReplacement {
        page_reference: Vec::<String>::new(),
        frame: pagereplacement.frame,
        status: pagereplacement.status,
        framecount: 0,
    };
    preoptimalpages =
        preoptimalpages.add_page_string(pagereplacement.page_reference, pagereplacement.framecount);
    let optimalpages = optimal_page::optimal(preoptimalpages);
    let processedpages = page_replacement::PageReplacement {
        page_reference: optimalpages.page_reference,
        frame: optimalpages.frame,
        status: optimalpages.status,
        framecount: optimalpages.framecount,
    };
    return processedpages;
}

fn lru(pagereplacement: page_replacement::PageReplacement) -> page_replacement::PageReplacement {
    let mut prelrupages = lru_page::page_replacement::PageReplacement {
        page_reference: Vec::<String>::new(),
        frame: pagereplacement.frame,
        status: pagereplacement.status,
        framecount: 0,
    };
    prelrupages =
        prelrupages.add_page_string(pagereplacement.page_reference, pagereplacement.framecount);
    let lrupages = lru_page::lru(prelrupages);
    let processedpages = page_replacement::PageReplacement {
        page_reference: lrupages.page_reference,
        frame: lrupages.frame,
        status: lrupages.status,
        framecount: lrupages.framecount,
    };
    return processedpages;
}
