fn main() {
    let mut num = 5;
    // Cria um ponteiro cru para num.
    let r1: *mut i32 = &mut num;

    // Bloco unsafe para desreferenciar o ponteiro.
    unsafe {
        *r1 += 1;
        println!("Valor modificado via ponteiro cru: {}", *r1);
    }
}
