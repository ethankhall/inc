fn build_help(available_commands: &Vec<&SystemCommand>) -> String {
    let mut help = String::new();
    write!(&mut help, "usage: inc [--verbose (-v)] <command> <args>\n").unwrap();
    write!(&mut help, "Available commands:\n").unwrap();

    for command in available_commands.iter() {
        write!(&mut help, "\t{}\n", command.alias).unwrap();
    }

    return help;
}
