use std::process::exit;

mod cli;

fn main() {
    /*
     * 1. rufile -> procurar makefile na raiz e executar o target main se existir
     * 1.1 rufile {target} -> mesma coisa porém executar o target
     * 1.2 rufile ... --quiet -q -> mesma coisa porém sem saída (apenas executar)
     * 1.3 rufile ... --file {file} -f {file} -> passando o arquivo
     * 1.4 rufile --version -v -> versão
     */

    let args = cli::get_arguments();

    if let None = args.get(0) {
        println!("executando target main...");
        exit(0)
    }

    match args[0].as_str() {
        "--help" | "-h" => cli::show_help(),
        "--version" | "-v" => cli::show_version(),
        arg => {
            // pegar target
            println!("{arg}")
        },
    }
}
