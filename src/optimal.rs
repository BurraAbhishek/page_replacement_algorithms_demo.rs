use std::collections::VecDeque;
#[path = "hit.rs"]
pub mod hit_rs;
#[path = "pagereplacement.rs"]
pub mod page_replacement;
use page_replacement::PageReplacement;

pub fn optimal(mut p: PageReplacement) -> PageReplacement {
    println!("Optimal Page replacement demonstration\n");
    // Logs all the pages used. Assuming each page to be represented as a string...
    let mut log: VecDeque<String> = VecDeque::<String>::new();
    // Copy all pages into log
    for i in 0..p.page_reference.len() - 1 {
        log.push_back(String::from(
            p.page_reference.get(i).unwrap_or(&String::new()),
        ));
    }
    // Traverse through each page in the page reference string
    for i in 0..p.page_reference.len() - 1 {
        // The previous frame
        let mut prev: Vec<String> = Vec::<String>::new();
        // The current frame
        let mut current: Vec<String> = Vec::<String>::new();
        // If not the first frame
        if i > 0 {
            // Copy the previous frame from the original frame to prev and current
            for j in 0..p.frame[i - 1].len() {
                let val = p.frame[i - 1].get(j);
                let strval = String::from(val.unwrap_or(&String::new()));
                prev.push(strval);
                let strval = String::from(val.unwrap_or(&String::new()));
                current.push(strval);
            }
            // If current does not have the page, it means page is not in memory
            // Lookup for the page
            if !current.contains(&p.page_reference[i]) {
                if current.len() == p.framecount.into() {
                    // The frame is filled. Replacement is necessary
                    let mut replaced = String::new();
                    let mut is_found: bool = true;
                    let mut find_index: usize = 0;
                    for j in 0..current.len() {
                        let strval = String::from(current.get(j).unwrap_or(&String::new()));
                        if !log.contains(&strval) && is_found {
                            replaced = strval;
                            is_found = false;
                        }
                    }
                    if is_found {
                        for j in 0..current.len() {
                            let strval = String::from(current.get(j).unwrap_or(&String::new()));
                            for k in 0..log.len() {
                                let found_strval =
                                    String::from(log.get(k).unwrap_or(&String::new()));
                                if found_strval == strval && found_strval != String::new() {
                                    find_index = k;
                                }
                            }
                        }
                        replaced = String::from(log.get(find_index).unwrap_or(&String::new()));
                    }
                    for j in 0..current.len() {
                        // Replace the page first logged into the memory
                        if current[j] == replaced {
                            let new_val = p.page_reference.get(i);
                            current[j] = String::from(new_val.unwrap_or(&String::new()));
                        }
                    }
                } else if current.len() < p.framecount.into() {
                    // The frame is not filled. Add the page to the frame.
                    let val = p.page_reference.get(i);
                    let strval = String::from(val.unwrap_or(&String::new()));
                    current.push(strval);
                }
            }
        } else {
            // First frame. Simply add the first page into memory.
            let val = p.page_reference.get(0);
            let strval = String::from(val.unwrap_or(&String::new()));
            current.push(strval);
        }
        // Generate if miss (page fault, not in memory) or hit (page in memory)
        p.status.push(hit_rs::is_hit(&current, &prev));
        // Remove the used frame from the log
        let _popped = log.pop_front();
        // Save the current frame
        p.frame.push(current);
    }
    return p;
}
