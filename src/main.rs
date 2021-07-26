#[path = "page.rs"] mod page;
use page as new_page;

fn main() {
    let demopage = new_page::Page {
        ..Default::default()
    };
    demopage.generate();
    println!("Execution successful. This work is still under development");
}
