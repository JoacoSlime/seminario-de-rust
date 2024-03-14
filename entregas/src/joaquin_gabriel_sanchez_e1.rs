use random_number::random;

pub fn get_par_random(arreglo: [u32; 5]) -> i64 {
    let r: usize = random!(0,arreglo.len()-1);
    if arreglo[r] % 2 == 0 {
        arreglo[r] as i64
    } else {
        -1
    }
}