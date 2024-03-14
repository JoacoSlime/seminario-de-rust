use std::io::Write;

pub fn ejercicio1() {
    let f : f64 = 2.48;
    print!("Ingrese un numero: ");
    std::io::stdout().flush().expect("El flush falló");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error al leer");
    let i : f64 = buf.trim().parse().expect("No es un número");
    println!("Suma: {f}+{i}={}",f+i);
    println!("Resta: {f}-{i}={}",f-i);
    println!("Multiplicación: {f}*{i}={}",f*i);
    println!("División: {f}/{i}={}",f/i);
}

pub fn ejercicio2() {
    let i:u32 = 255;
    println!("{} en hexadecimal es: {}", i, format!("{i:X}").as_str());
}

pub fn ejercicio3() {
    let boleano = true;
    print!("Ingrese un valor booleano: ");
    std::io::stdout().flush().expect("El flush falló");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error al leer");
    let boleano2 : bool = buf.trim().parse().expect("No es un booleando");
    println!("{} AND {} == {}", boleano, boleano2, boleano & boleano2);
    println!("{} OR {} == {}", boleano, boleano2, boleano | boleano2);
}

pub fn ejercicio4() {
    let tupla = ("Hola",5,true);
    println!("La tupla contiene: ({},{},{})",tupla.0,tupla.1,tupla.2);
}

pub fn ejercicio5() {
    let s = "Hola";
    println!("Ingrese una cadena para concatenar a: \"{}\"", s);
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error al leer");
    println!("{}{}",s,buf.trim());
}

pub fn ejercicio6 () {
    let u:u32 = 2;
    print!("Ingrese valor a sumar a {}: ", u);
    std::io::stdout().flush().expect("El flush falló");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error al leer");
    let i: i32 = buf.trim().parse().expect("Error: No es un Numero");
    let suma: i32 = u as i32 + i;
    println!("El valor de la suma elevado al cuadrado es: {}",suma.pow(2));
}