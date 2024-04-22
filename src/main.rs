fn main() {
    // VARIÁVEIS PRIMITIVAS
    const CIDADE: &str = "São Paulo";
    let estado: &str = "SP";

    print!("Olá, ");
    println!("{} do estado {}!", CIDADE, estado);

    // let numero_sem_negativo: u8 = -1; ERROR
    let numero_sem_negativo: u8 = 255;
    // let numero_sem_negativo: u8 = 257 ERROR;
    let numero_com_negativo: i8 = -128;
    // let numero_com_negativo: i8 = -129; ERROR
    let idade: u16 = 469;

    println!("Você tem {} anos.", idade);

    let valor_apartamento: f32 = 1_000_000.00;
    // let valor_apartamento: f32 = 1000000.00;

    let devo_comprar_o_apartamento: bool = true;
    // let devo_comprar_o_apartamento: bool = false;

    println!("O valor do apartamento é R$ {:.2}", valor_apartamento);
    println!("Devo comprar o apartamento? {}", devo_comprar_o_apartamento);

    // OPERADORES MATEMÁTICOS
    let sum = 5 + 10;
    let sub = 10 - 5;
    let div = 10 / 5;
    let mul = 10 * 5;

    println!("5 + 10 = {}", sum);
    println!("10 - 5 = {}", sub);
    println!("10 / 5 = {}", div);
    println!("10 * 5 = {}", mul);

    //MATRIZES
    let matriz = [1, 2, 3, 4, 5];
    let matriz_texto = ["um", "dois", "três", "quatro", "cinco"];

    println!("Matriz é {:?}", matriz);
    println!("O terceiro elemento da matriz é {}", matriz[3]);
    println!("Matriz de texto é {:?}", matriz_texto);
    println!("O quarto elemento da matriz de texto é {}", matriz_texto[3]);

    // TUPLES
    let tupla: (i32, &str, f64, bool) = (1, "dois", 3.0, true);
    let (a, b, c, d) = tupla;

    println!("Tupla é {:?}", tupla);
    println!("O primeiro elemento da tupla é {}", tupla.0);
    println!("O segundo elemento da tupla é {}", tupla.1);
    println!("O segundo elemento da tupla é {}", b);

    // FUNÇÕES
    let resultado = soma(5, 10);
    println!("O resultado da soma é {}", resultado);
    let resultado_com_return = soma_com_return(5, 10);
    println!("O resultado da soma com return é {}", resultado_com_return);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}
fn soma_com_return(a: i32, b: i32) -> i32 {
    return a + b;
}
