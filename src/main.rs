use std::io;

fn main() {
    println!("{:-^40}", "INÍCIO");

    let mut name = String::new();
    println!("Digite seu nome:");

    io::stdin()
        .read_line(&mut name)
        .expect("Erro ao ler nome");

    let nums: Vec<&str> = name
        .split(",")
        .map(|c| c.trim())
        .collect();

    let joined_name = nums.join("");

    let name_length = joined_name.trim().chars().count();
    let name_uppercase = joined_name.trim().to_uppercase();
    let name_without_r_letter = joined_name.trim().replace("r", " ");

    println!("Olá, {}, seja bem vindo(a) ", joined_name);
    println!("Seu nome tem {} letras", name_length);
    println!("Seu nome maiúsculo é {}", name_uppercase);
    println!("Seu sem a letra R é {}", name_without_r_letter);
    println!("Seu nome tem {:?} palavras", nums);

    println!("{:-^40}", "TÉRMINO");
}
