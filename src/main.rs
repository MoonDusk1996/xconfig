mod configs;
mod menu;
mod system;

fn main() {
    //create xconfig if do not exist
    configs::create_xconfig();

    //print sysinfo
    if let Some(cpu_info) = system::get_cpu_info() {
        println!("CPU info:");
        println!("\tProcessor Name: {}", cpu_info.name);
        println!("\tNumber of Cores: {}\n", cpu_info.num_cores);
    }

    //cli menu
    menu::show_menu();
}
