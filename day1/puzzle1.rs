use std::fs;

fn main() {
    let file_path = "src/input.txt";
    let entrada = read_file_to_vector(file_path);
    let tamanho_entrada = entrada.len() as i32;
    let mut count = 0i32;
    let mut resultados_linha: Vec<i32> = Vec::new();
    let mut aux2: i32 = 0;
    loop {
        let mut aux = "".to_string();
        let mut palavra = entrada[count as usize].to_string();

        aux = retira_numeros(&palavra.clone());
        match aux.parse::<i32>() {
            Ok(parsed_value) => {
                aux2 = parsed_value;
                resultados_linha.push(aux2);
            }
            Err(err) => {
                //println!("Não foi possível converter: {}", err);
            }
        }

        println!("Palavra encerrada: {}", aux2);
        //println!("{:?}", resultados_linha);

        println!("{}", count);
        count += 1;
        if count == tamanho_entrada {
            let resultado_final: i32 = resultados_linha.iter().sum();
            println!("Chave encerrada: {} ", resultado_final);
            break;
        }
    }
}

fn retira_numeros(palavra: &str) -> String {
    let caracteres_numericos: String = palavra.chars().filter(|&c| c.is_numeric()).collect();

    if let Some(primeiro) = caracteres_numericos.chars().next() {
        if let Some(ultimo) = caracteres_numericos.chars().last() {
            return format!("{}{}", primeiro, ultimo);
        }
    }
    String::new()
}

fn read_file_to_vector(file_path: &str) -> Vec<String> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        return contents.lines().map(String::from).collect();
    }

    Vec::new()
}
