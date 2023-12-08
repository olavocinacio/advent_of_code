use regex::Regex;
use std::fs;

fn main() {
    let file_path = "src/input2.txt";
    let entrada = read_file_to_vector(file_path);
    let mut resultado: Vec<String> = Vec::new();
    for palavra in entrada {
        resultado.push(converte_numeros_extenso(palavra));
    }
    day1(resultado);
}

fn converte_numeros_extenso(entrada: String) -> String {
    let numeros_extenso = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut saida = entrada.clone();
    for num in numeros_extenso {
        let re = Regex::new(num).unwrap();
        let text_to_num = numeros_extenso
            .iter()
            .position(|&x| x == num)
            .map(|pos| (pos + 1).to_string());
        let substituto = format!("{}{}", num, text_to_num.unwrap().as_str());
        let resultado = re.replace_all(&saida, substituto);
        saida = resultado.to_string();
        println!("{:?}", saida);
    }
    return saida;
}

fn day1(entrada: Vec<String>) {
    let tamanho_entrada = entrada.len() as i32;
    let mut count = 0i32;
    let mut resultados_linha: Vec<i32> = Vec::new();
    let mut aux2: i32 = 0;
    loop {
        if count == tamanho_entrada {
            let resultado_final: i32 = resultados_linha.iter().sum();
            println!("Chave encerrada: {} ", resultado_final);
            break;
        }
        let palavra = entrada[count as usize].to_string();
        let aux = retira_numeros(&palavra.clone());
        match aux.parse::<i32>() {
            Ok(parsed_value) => {
                aux2 = parsed_value;
                resultados_linha.push(aux2);
            }
            Err(err) => {
                println!("Não foi possível converter: {}", err);
            }
        }

        println!("Palavra encerrada: {}", aux2);

        count += 1;
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