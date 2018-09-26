#[cfg(test)]
pub mod test {
    use core::config::*;
    use serde_yaml;

    #[test]
    fn test_can_find_list_of_command() {
        let foo_commands = 
"exec:
    foo:
        commands: 
            - bar
            - baz";
        let result = serde_yaml::from_str::<ProjectConfig>(foo_commands).unwrap();
        assert!(result.exec.contains_key("foo"), "foo didn't exist");

        let foo = result.exec.get("foo").unwrap();
        let foo_commands = foo.clone().commands;
        assert_eq!(foo_commands.len(), 2);
        assert_eq!(foo_commands.get(0).unwrap(), &Commands::CommandList(String::from("bar")));
        assert_eq!(foo_commands.get(1).unwrap(), &Commands::CommandList(String::from("baz")));
        assert_eq!(foo.clone().ignore_failures, false);
    }

    #[test]
    fn test_inherited_commands() {
        let yaml1 = 
"exec:
    foo:
        commands: 
            - bar1
            - baz1";
        let yaml1 = serde_yaml::from_str::<ProjectConfig>(yaml1).unwrap();

        let yaml2 = 
"exec:
    bar:
        commands: 
            - bar2
            - baz2";
        let yaml2 = serde_yaml::from_str::<ProjectConfig>(yaml2).unwrap();

        let yaml3 = 
"exec:
    baz:
        commands: 
            - bar3
            - baz3
            - flig3
    foo:
        commands:
            - nope";
        let yaml3 = serde_yaml::from_str::<ProjectConfig>(yaml3).unwrap();

        let config_container = ConfigContainer {
            project_config: vec![ConfigWithPath::no_file(yaml1),ConfigWithPath::no_file( yaml2), ConfigWithPath::no_file(yaml3)],
            home_config: ConfigWithPath::no_file(HomeConfig { checkout: CheckoutConfigs { default_provider: None } }),
        };

        let exec_configs = config_container.get_exec_configs();

        assert!(exec_configs.commands.contains_key("foo"), "has foo key");
        assert!(exec_configs.commands.contains_key("bar"), "has foo bar");
        assert!(exec_configs.commands.contains_key("baz"), "has foo baz");
        assert_eq!(exec_configs.commands.len(), 3);

        let foo_command = exec_configs.commands.get("foo").unwrap().clone().commands;
        assert_eq!(foo_command.len(), 2);
        assert_eq!(foo_command.get(0), Some(&Commands::CommandList(String::from("bar1"))));
        assert_eq!(foo_command.get(1), Some(&Commands::CommandList(String::from("baz1"))));

        let bar_command = exec_configs.commands.get("bar").unwrap().clone().commands;
        assert_eq!(bar_command.len(), 2);
        assert_eq!(bar_command.get(0), Some(&Commands::CommandList(String::from("bar2"))));
        assert_eq!(bar_command.get(1), Some(&Commands::CommandList(String::from("baz2"))));

        let baz_command = exec_configs.commands.get("baz").unwrap().clone().commands;
        assert_eq!(baz_command.len(), 3);
        assert_eq!(baz_command.get(0), Some(&Commands::CommandList(String::from("bar3"))));
        assert_eq!(baz_command.get(1), Some(&Commands::CommandList(String::from("baz3"))));
        assert_eq!(baz_command.get(2), Some(&Commands::CommandList(String::from("flig3"))));
    }
}
