use crate::tp4::EsPrimo;

mod tp1;
mod tp2;
mod tp3;
mod tp4;
mod tp5;

#[allow(unused)]
fn menu() {
    println!("================================================");
    println!("| 1) Práctica 1.");
    println!("| 2) Práctica 2.");
    println!("| 3) Práctica 3 (Falta Test de ej 10).");
    println!("| 4) Práctica 4 (Falta Test de ej 10).");
    println!("|");
    println!("| 0) Salir.");
    println!("================================================");
    println!();
    println!("Elige práctica a probar: ");
}

fn esperar(){
    println!();
    println!("Pulsa Enter para continuar.");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Error al leer");
}

fn practica1() {
    println!();
    tp1::ejercicio1();
    println!();
    tp1::ejercicio2();
    println!();
    tp1::ejercicio3();
    println!();
    tp1::ejercicio4();
    println!();
    tp1::ejercicio5();
    println!();
    tp1::ejercicio6();
}

fn practica2() {
    println!();
    let numero = 7;
    match tp2::es_par(numero) {
        true => println!("{} es par.", numero),
        false => println!("{} no es par.", numero),
    }
    println!();

    match tp2::es_primo(numero) {
        true => println!("{} es primo.", numero),
        false => println!("{} no es primo.", numero),
    }
    println!();

    let arreglo = [9,8,4,5,3,8,4,6,9,10];
    println!("La suma de los pares del arreglo {:?} es {}", arreglo, tp2::suma_pares(arreglo));
    println!("El arreglo {:?} tiene {} impares", arreglo, tp2::cantidad_impares(arreglo));
    println!();

    let flotantes = [1.54,2.465,2.246,8.546,5.654,5.545,2.5435,7.54,4.456,9.546];
    println!("El arreglo {:?} duplicado es {:?}", flotantes, tp2::duplicar_valores(flotantes));
    println!();

    let cadenas = [
        "Hola".to_string(), 
        "Adios".to_string(), 
        "Rust".to_string(), 
        "Python".to_string(), 
        "JavaScript".to_string()];
    println!("Las cadenas {:?} tienen los tamaños {:?}", cadenas, tp2::longitud_de_cadenas(&cadenas));
    println!();

    let fs1 = [1.54,2.465,2.246,8.546,5.654,5.545,2.5435,7.54,4.456,9.546];
    let fs2 = [3.08, 4.93, 4.492, 17.092, 11.308, 11.09, 5.087, 15.08, 8.912, 19.092];
    println!("La suma de las cadenas {:?} y {:?} es {:?}", fs1, fs2, tp2::sumar_arreglos(fs1, fs2));
    println!();

    let arreglo = [9,8,4,5,3,8,4,6,9,10];
    println!("La cantidad de numeros del arreglo {:?} en el rango 5 a 8 es {}", arreglo, tp2::cantidad_en_rango(arreglo, 5, 8));
    println!();

    let cadenas = [
        "Hola".to_string(), 
        "Adios".to_string(), 
        "Rust".to_string(), 
        "Python".to_string(), 
        "JavaScript".to_string()];
    println!("La cantidad de cadenas en {:?} mayor a 5 es {}", cadenas, tp2::cantidad_de_cadenas_mayor_a(&cadenas, 5));
    println!();

    let mut arreglo = [9,8,4,5,3,8,4,6,9,10];
    print!("El arreglo {:?} ", arreglo);
    tp2::multiplicar_valores(&mut arreglo, 5);
    println!("queda : {:?}", arreglo);
    println!();

    let mut arreglo = [9,8,4,5,3,8,4,6,9,10];
    print!("El arreglo {:?} ", arreglo);
    tp2::reemplazar_pares(&mut arreglo);
    println!("reemplazando los pares queda: {:?}", arreglo);
    println!();

    let mut nombres = [
        "Josías".to_string(),
        "Alberto".to_string(),
        "José".to_string(),
        "Gabriel".to_string(),
        "Nahuel".to_string(),
    ];
    print!("Las cadenas {:?} ", nombres);
    tp2::ordenar_nombres(&mut nombres);
    println!("ordenando los nombres queda: {:?}", nombres);
    println!();

    let mut f = 31.64;
    print!("El flotante {} ", f);
    tp2::incrementar(&mut f);
    println!("+ 1.0 queda {}", f);
}

fn practica3() {
    tp3::ejercicio1();
    tp3::ejercicio2();
    tp3::ejercicio3();
    tp3::ejercicio4();
    tp3::ejercicio5();
    tp3::ejercicio6();
    tp3::ejercicio7();
    tp3::ejercicio8();
    tp3::ejercicio9();
}

fn practica4() {
    let num = 11;
    println!("{} es primo?: {}", num, num.es_primo());
}

fn main() {
    
    'menu: loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        menu();
        let mut seleccion = String::new();
        std::io::stdin().read_line(&mut seleccion).expect("Error al leer");
        let s: u8 = seleccion.trim().parse().expect("Error al parsear");
        match s{
            1 => {practica1(); esperar();},
            2 => {practica2(); esperar();},
            3 => {practica3(); esperar();},
            0 => break 'menu,
            _ => (),
        }
    }
}
