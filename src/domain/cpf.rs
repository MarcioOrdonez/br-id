use rand::{thread_rng, Rng};

pub struct Cpf;

impl Cpf {
    pub fn generate() -> String {
        let mut rng = thread_rng();
        let n: Vec<u32> = (0..9).map(|_| rng.gen_range(0..9)).collect();
        let mut k =
            n[1] * 9 + n[2] * 8 + n[3] * 7 + n[4] * 6 + n[5] * 5 + n[6] * 4 + n[7] * 3 + n[8] * 2;
        let mut j = 11 - ((n[0] * 10 + k) % 11);

        j = if j >= 10 { 0 } else { j };

        k = 11 - ((k + j) % 11);
        k = if k >= 10 { 0 } else { k };

        let cpf = format!(
            "{:?}{:?}{:?}.{:?}{:?}{:?}.{:?}{:?}{:?}-{:?}{:?}",
            n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], j, k
        );
        cpf
    }
}
