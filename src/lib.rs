pub struct Histogram {
    num_entries: usize,
    labels: Vec<String>,
    label_len: usize,
    values: Vec<usize>,
    pub max_width: usize,
}

pub fn init() -> Histogram {
    return Histogram {
        num_entries: 0,
        label_len: 1,
        values: Vec::new(),
        labels: Vec::new(),
        max_width: 0,
    };
}

impl Histogram {
    pub fn add_entry(&mut self, label: String, value: usize) {
        if label.len() > self.label_len {
            self.label_len = label.len();
        }

        self.values.push(value);
        self.labels.push(label);
        self.num_entries += 1;
    }

    pub fn draw(&self) {
        let max = self.values.iter().max().unwrap();
        let width = if self.max_width > 0 {
            self.max_width
        } else {
            console::Term::stdout().size().0 as usize - (self.label_len + 2)
        };
        let dw = max / width;

        for x in 0..self.num_entries {
            println!();
            let s = format!(
                "{label:>width$}",
                label = self.labels[x],
                width = self.label_len
            );
            print!("{0}{1}", s, "|");

            for y in 0..width {
                if self.values[x] > dw * y {
                    print!("#");
                } else {
                    break;
                }
            }
        }

        println!();
    }
}

pub fn draw_from_array(arr: Vec<usize>, size: usize) {
    let max = arr.iter().max().unwrap();
    let num_digits = ((size as f32).log10() + 1.0).floor() as usize;
    let width = console::Term::stdout().size().1 as usize - (num_digits + 2);
    let dw = max / width;

    for x in 0..size {
        let s: String = format!("{:0width$}", x, width = num_digits);
        println!();
        println!("{0}{1}", s, "|");

        for y in 0..(width - num_digits + 2) {
            if arr[x as usize] > dw * y {
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
