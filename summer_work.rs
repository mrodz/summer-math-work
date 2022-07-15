pub fn main() {
    {
        println!(
            "{}",
            LinearEquation::new((0, 0), (3, 2)) == LinearEquation::new((-3, -2), (0, 0))
        );
    }

    println!("\n~~~\n");

    {
        let arr = vec![12, 15, 20, 32, 24, 22, 20, 37, 17, 50];

        println!("the mean is {}", list_ops::mean(&arr));
        println!("the median is {}", list_ops::median(&arr));
        println!("the mode is {}", list_ops::mode(&arr));
        println!("the range is {:?}", list_ops::range(&arr));
    }

    println!("\n~~~\n");

    {
        let (a, b, c) = (1f32, -6f32, -7f32);

        let res: [f32; 2] = quadratic(a, b, c);
        println!("x-intercepts: {:?}", res);

        let x = res.iter().sum::<f32>() / (res.len() as f32);
        println!("axis of symmetry: {}", x);

        let top = a * x.powi(2) + b * x + c;
        println!("highest point: {}", top);
    }
}

struct LinearEquation {
    p1: (i32, i32),
    p2: (i32, i32),
}

impl std::cmp::PartialEq for LinearEquation {
    fn eq(&self, other: &Self) -> bool {
        self.slope() == other.slope() && self.b() == other.b()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl ToString for LinearEquation {
    fn to_string(&self) -> String {
        let b = self.b();
        format!(
            "y={}x{}{}",
            self.slope().0,
            if f32::is_sign_positive(b) { "+" } else { "" },
            self.b()
        )
    }
}

impl std::fmt::Debug for LinearEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl LinearEquation {
    pub fn slope(&self) -> (f32, (i32, i32)) {
        let s: (i32, i32) = (self.p2.1 - self.p1.1, self.p2.0 - self.p1.0);
        (s.0 as f32 / s.1 as f32, s)
    }

    pub fn b(&self) -> f32 {
        -self.slope().0 * self.p1.0 as f32 + self.p1.1 as f32
    }

    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        Self { p1: p1, p2: p2 }
    }

    pub fn y_from_x(&self, x: i32) -> i32 {
        (self.slope().0 * x as f32 + self.b()) as i32
    }

    pub fn x_from_y(&self, y: i32) -> i32 {
        ((y as f32 - self.b()) / self.slope().0) as i32
    }
}

mod list_ops {
    pub fn range(v: &Vec<i32>) -> i32 {
        let r: (i32, i32) = (*v.iter().min().expect("min"), *v.iter().max().expect("max"));
        r.1 - r.0
    }

    pub fn mode(v: &Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<&i32, i32> = HashMap::new();
        for i in v {
            if map.contains_key(i) {
                map.insert(i, map.get(i).unwrap() + 1);
            } else {
                map.insert(i, 0);
            }
        }
        type Pair<'a> = (&'a &'a i32, &'a i32);
        let mut keys: Vec<Pair> = map.iter().collect();
        keys.sort_by(|a: &Pair, b: &Pair| a.1.partial_cmp(b.1).unwrap());
        **keys.last().unwrap().0
    }

    pub fn median(v: &Vec<i32>) -> f32 {
        let mut v = v.clone();
        v.sort();

        let mid = *v.get(v.len() / 2).expect("always returns a value");
        if v.len() & 1 == 0 {
            mean(&[mid, *v.get(v.len() / 2 - 1).unwrap()].to_vec())
        } else {
            mid as f32
        }
    }

    pub fn mean(v: &Vec<i32>) -> f32 {
        v.iter().map(|n| *n as f32).sum::<f32>() / (v.len() as f32)
    }
}

pub fn quadratic(a: f32, b: f32, c: f32) -> [f32; 2] {
    let s = ((b * b) - 4f32 * a * c).sqrt();
    let a = 2f32 * a;
    [(-b + s) / a, (-b - s) / a]
}
