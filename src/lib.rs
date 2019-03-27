
struct Histogram {
    num_entries: i32,
    labels: Vec<str>,
    label_len: i32,
    values: Vec<i32>,
    entry_limit: i32
}

impl Histogram {

    pub fn init() {
        return Histogram{
            num_entries: 0,
            entry_limit: 0,
            label_len: 1;
            values: vec![],
            labels: vec![],
        };
    }
    
    pub fn add_entry(self, label: str, num_digits: i32) -> None {
        self.values.push(value);
        self.labels.push(label);
        self.num_entries++;

        if label.length > self.label_len {
            self.label_len = label.length;
        }
    }

    pub fn draw(self) -> None {

        let max = self.values.iter().max();
        let width = termion.terminal_size() - (self.label_len + 2);
        let dw = max / width;

        for x in 0..num_entries {
            println!();
            let mut s = format!(":width$", self.labels[x], width=self.label_len);
            println!(s + "|");

            for y in 0..width {
                if self.values[x]  > dw * y {
                    print!("#");
                } else {
                    break; 
                }
            }
        }

        println!();
    }

}

fn draw_from_array(arr: Vec<i32>, size: i32) {
    let max = arr.iter().max();
    let num_digits = ((size as f32).log10() + 1).floor();
    let width = termion.terminal_size.unwrap().1 - (num_digits + 2);
    let dw = max / width;

    for x in 0..size {

        let mut s: str = format!("{:0width$}", x, width=num_digits);
        println!();
        println!(s + "|");

        for y in 0..(width - num_digits + 2) {
            if arr[x] > dw * y {
                print!("#");
            } else {
                break; 
            }
        }
    }

    println!();

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
