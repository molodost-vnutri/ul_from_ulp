use std::process::Command;
use colored::Colorize;

pub fn clear_screen() {
    let logo = format!("{}{}{}{}{}{}"
    , "██╗░░░░░░█████╗░██╗░░░░░███████╗████████╗███████╗░█████╗░███╗░░░███╗\n██║░░░░░██╔══██╗██║░░░░░╚════██║╚══██╔══╝██╔════╝██╔══██╗████╗░████║\n██║░░░░░██║░░██║██║░░░░░░░███╔═╝░░░██║░░░█████╗░░███████║██╔████╔██║\n██║░░░░░██║░░██║██║░░░░░██╔══╝░░░░░██║░░░██╔══╝░░██╔══██║██║╚██╔╝██║\n███████╗╚█████╔╝███████╗███████╗░░░██║░░░███████╗██║░░██║██║░╚═╝░██║\n╚══════╝░╚════╝░╚══════╝╚══════╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝╚═╝░░░░░╚═╝\n".green()
    , "         Сделал molodost vnutri для форума zekenka.guru\n         Контакты и ссылки для фидбека:\n"
    , format!("             [{}]=> @M0l0d0st_vnutri\n", "Telegram".blue())
    , format!("             [{}]=> https://zelenka.guru/members/3060240/\n", "Форум".green())
    , format!("             [{}]=> https://zelenka.guru/threads/6638820/\n", "Тема на форуме".green())
    , format!("             [{}]=> https://github.com/molodost-vnutri/ulp_normalize/\n\n", "Github".bright_black())
    );

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
    println!("{}",logo);
}