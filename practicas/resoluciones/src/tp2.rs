#[allow(unused)]
pub fn es_par(numero: i32) -> bool{
    match numero%2 {
        0 => true,
        _ => false,
    }
}

pub fn es_primo(numero: i32) -> bool{
    if numero > 0 {
        let mut es_primo = true;
        let mut uno_menos = numero - 1;
        while es_primo && uno_menos > 1 {
            if numero % uno_menos == 0 {
                es_primo = false; 
            }
            uno_menos -= 1;
        }
        es_primo
    } else {
        false
    }
}

pub fn suma_pares(arreglo: [i32; 10]) -> i32 {
    let mut suma = 0;
    for n in arreglo {
        suma += n;
    }
    suma
}

pub fn cantidad_impares(arreglo: [i32; 10]) -> i32 {
    let mut cont = 0;
    for n in arreglo {
        cont = if !es_par(n) {
            cont + 1
        } else {
            cont
        }
    }
    cont
}

pub fn duplicar_valores (arreglo: [f64; 10]) -> [f64; 10] {
    let mut nuevo_arreglo = arreglo;
    for i in 0..nuevo_arreglo.len() {
        nuevo_arreglo[i] *= 2.0;
    } 
    nuevo_arreglo
}

pub fn longitud_de_cadenas (cadenas: &[String; 5]) -> [i32; 5] {
    let mut long: [i32; 5] = [0,0,0,0,0];
    for i in 0..cadenas.len() {
        long[i] = cadenas[i].len() as i32;
    }
    long
}

pub fn sumar_arreglos (arreglo1: [f64; 10], arreglo2: [f64; 10]) -> [f64; 10] {
    let mut nuevo_arreglo = arreglo1;
    for i in 0..arreglo2.len() {
        nuevo_arreglo[i] += arreglo2[i];
    }
    nuevo_arreglo
}

pub fn cantidad_en_rango (arreglo: [i32; 10], inferior: i32, superior: i32) -> u32 {
    let mut ret = 0;
    for i in arreglo {
        if (inferior<i) && (i<superior) {
            ret+=1;
        }
    };
    ret
}

pub fn cantidad_de_cadenas_mayor_a (cadenas: &[String; 5], limite: usize) -> u32 {
    let mut cont = 0;
    for s in cadenas.clone() {
        if s.len() >= limite {
            cont += 1
        };
    }
    cont
}

pub fn multiplicar_valores (arreglo: &mut [i32; 10], factor: i32) {
    for i in 0..arreglo.len() {
        arreglo[i]*=factor;
    }
}

pub fn reemplazar_pares (arreglo: &mut [i32; 10]) {
    for i in 0..arreglo.len() {
        if es_par(arreglo[i]) {
            arreglo[i] = -1;
        }
    }
}

pub fn ordenar_nombres (cadenas: &mut [String;5]) {
    cadenas.sort();
}

pub fn incrementar (f: &mut f64) {
    *f += 1.0;
}