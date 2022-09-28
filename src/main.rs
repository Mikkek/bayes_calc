fn main() {
    let p_b_as = vec!(0.5, 0.2, 0.05);
    let p_as = vec!(0.3, 0.1, 0.6);

    let t1 = Values::from_even_dist(&p_b_as);

    let t2 = Values::from_vecs(&p_as, &p_b_as);

    let res1 = bayes(t1);
    let res2 = bayes(t2);

    println!("frac:\t{}\nnum:\t{}\n", res1.0, res1.1);
    println!("frac:\t{}\nnum:\t{}", res2.0, res2.1);

}

fn bayes_bin(p_a: f64, p_b_a: f64, ) -> (String, f64){
    let p_na = 1.0 - p_a;
    let p_b_na = 1.0 - p_b_a;

    let numerator = p_b_a * p_a;
    let denominator = p_b_a * p_a + p_b_na * p_na;

    let frac = format!("{} / {}", numerator, denominator);

    (frac, numerator / denominator)
}

fn bayes(values: Values) -> (String, f64) {
    let top = values.get(0);
    
    let numerator = top.0 * top.1;
    let mut denominator = 0.0;

    for i in 0..values.len() {
        let (p_a, p_b_a) = values.get(i);

        denominator += p_a * p_b_a;
    }

    let frac = format!("{} / {}", numerator, denominator);

    assert!(denominator != 0.0);
    (frac, numerator / denominator)
}

struct Values {
    p_as: Vec<f64>,
    p_b_as: Vec<f64>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            p_as: Vec::new(),
            p_b_as: Vec::new(),
        }
    }

    pub fn from_vecs(p_a_vec: &Vec<f64>, p_b_a_vec: &Vec<f64>) -> Self {
        Self {
            p_as:   p_a_vec.clone(),
            p_b_as: p_b_a_vec.clone(),
        }
    }

    pub fn from_even_dist(p_b_a_vec: &Vec<f64>) -> Self {
        let p_as = vec![1.0 / p_b_a_vec.len() as f64; p_b_a_vec.len()];
        
        Self {
            p_as,
            p_b_as: p_b_a_vec.clone(),
        }
    }

    pub fn get(&self, index: usize) -> (f64, f64) {
        (self.p_as[index], self.p_b_as[index])
    }

    pub fn len(&self) -> usize {
        self.p_as.len()
    }
}
