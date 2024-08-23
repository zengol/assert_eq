fn main() {
    let a = 3;
    let b = 1 + 2; // b también es 3

    assert_eq!(a,b); //hasta aqui todo bien
    
    
    let b = 4;//cambiamos el valor de b 
              //utilizando "Sombra de variables"

    // Aserción con mensaje personalizado se imprime al no ser iguales.
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
}