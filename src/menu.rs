use crate::{configs, system};
use dialoguer::Select;

pub fn show_menu() {
    loop {
        let main_menu_options = ["CPU usage", "GPU usage", "Exit"];
        let main_menu_selection = Select::new()
            .with_prompt("Choose an option")
            .items(&main_menu_options)
            .default(0)
            .interact()
            .unwrap_or_else(|e| {
                eprintln!("Erro ao interagir com o menu principal: {}", e);
                std::process::exit(1);
            });

        match main_menu_selection {
            0 => {
                if let Some(cpu_info) = system::get_cpu_info() {
                    let mut cpu_options: Vec<String> =
                        (1..=cpu_info.num_cores).map(|i| i.to_string()).collect();
                    cpu_options.insert(0, "disable".to_string());
                    cpu_options.push("back".to_string());

                    let cpu_selection = Select::new()
                        .items(&cpu_options)
                        .default(0)
                        .interact()
                        .unwrap_or_else(|e| {
                            eprintln!("Erro ao interagir com o submenu CPU: {}", e);
                            std::process::exit(1);
                        });

                    match cpu_selection {
                        i if (0..=cpu_info.num_cores as usize).contains(&(i)) => {
                            println!("Using {} CPU core(s)\n", cpu_selection);
                            // Perform actions based on the submenu selection
                            // ...
                            configs::update_xmrig_config(cpu_selection as u32);
                            // Adicione um break aqui para retornar ao loop principal
                            // break;
                        }
                        _ => {
                            println!("Invalid selection. Returning to main menu.");
                        }
                    }
                };
            }
            // Caso 1 (submenu GPU) está comentado, remova o comentário quando for implementado
            1 => {
                // Código para submenu GPU aqui
                // ...
                println!("GPU");
                // Adicione um break aqui para retornar ao loop principal
                // break;
            }
            2 => {
                println!("Goodbye!");
                break; // Exit the program
            }
            _ => {
                eprintln!("Seleção inesperada no menu principal. Saindo...");
                std::process::exit(1);
            }
        }
    }
}
