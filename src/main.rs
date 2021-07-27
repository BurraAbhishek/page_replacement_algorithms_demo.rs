#[path = "input.rs"]
mod input_rs;
#[path = "page.rs"]
mod page_rs;

fn main() {
    let demopage = page_rs::Page {
        ..Default::default()
    };
    let page2 = demopage.generate();
    println!("Number of frames: ?");
    let _frames: u8 = input_rs::input_u8();
    let _pagestring = page2.get_page_string();
    println!("Execution successful. This work is still under development");
}
