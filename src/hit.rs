struct HitStatus {
    hit: String,
    miss: String,
}

impl Default for HitStatus {
    fn default() -> HitStatus {
        HitStatus {
            hit: "Hit ".to_string(),
            miss: "Miss".to_string(),
        }
    }
}

pub fn is_hit(current: &Vec<String>, prev: &Vec<String>) -> String {
    let mut status = "Hit";
    if prev.len() != current.len() {
        status = "Miss";
    } else {
        let x = current.len();
        for i in 0..x {
            if prev[i] != current[i] {
                status = "Miss";
            }
        }
    }
    let result = HitStatus {
        ..Default::default()
    };
    if status == "Hit" {
        return result.hit;
    } else if status == "Miss" {
        return result.miss;
    } else {
        panic!("Program error");
    }
}
