#[cfg(test)]
pub mod test {
    use core::config::*;
    use toml::Value;

    #[test]
    fn test_can_find_single_command() {
        let foo_commands = "[foo]
        commands = 'bar'".parse::<Value>().unwrap();
        let foo_commands = foo_commands.as_table();

        let results = prase_exec_table(foo_commands);
        assert!(results.contains_key("foo"), "foo didn't exist");
        let foo = results.get("foo").unwrap();
        assert!(foo.commands.is_some(), "foo should have commands");
        let foo_commands = foo.clone().commands.unwrap();
        assert_eq!(foo_commands.len(), 1);
        assert_eq!(foo_commands.get(0).unwrap(), &String::from("bar"));
        assert_eq!(foo.clone().ignore_failures, false);
    }

    #[test]
    fn test_can_find_list_of_command() {
        let foo_commands = "[foo]
        commands = ['bar', 'baz']".parse::<Value>().unwrap();
        let foo_commands = foo_commands.as_table();

        let results = prase_exec_table(foo_commands);
        assert!(results.contains_key("foo"), "foo didn't exist");
        let foo = results.get("foo").unwrap();
        assert!(foo.commands.is_some(), "foo should have commands");
        let foo_commands = foo.clone().commands.unwrap();
        assert_eq!(foo_commands.len(), 2);
        assert_eq!(foo_commands.get(0).unwrap(), &String::from("bar"));
        assert_eq!(foo_commands.get(1).unwrap(), &String::from("baz"));
        assert_eq!(foo.clone().ignore_failures, false);
    }

    #[test]
    fn test_inherited_commands() {
        let toml1 = "[exec.foo]
        commands = ['bar1', 'baz1']".parse::<Value>().unwrap();

        let toml2 = "[exec.bar]
        commands = ['bar2', 'baz2']".parse::<Value>().unwrap();

        let toml3 = "[exec.baz]
        commands = ['bar3', 'baz3', 'flig3']
        
        [exec.foo]
        commands = 'nope'".parse::<Value>().unwrap();

        let config_container = ConfigContainer { project_config: vec![toml1, toml2, toml3], home_config: vec![] };

        let exec_configs = config_container.get_exec_configs();

        assert!(exec_configs.commands.contains_key("foo"), "has foo key");
        assert!(exec_configs.commands.contains_key("bar"), "has foo bar");
        assert!(exec_configs.commands.contains_key("baz"), "has foo baz");
        assert_eq!(exec_configs.commands.len(), 3);

        let foo_command = exec_configs.commands.get("foo").unwrap().clone().commands.expect("foo command exists");
        assert_eq!(foo_command.len(), 2);
        assert_eq!(foo_command.get(0), Some(&String::from("bar1")));
        assert_eq!(foo_command.get(1), Some(&String::from("baz1")));

        let bar_command = exec_configs.commands.get("bar").unwrap().clone().commands.expect("bar command exists");
        assert_eq!(bar_command.len(), 2);
        assert_eq!(bar_command.get(0), Some(&String::from("bar2")));
        assert_eq!(bar_command.get(1), Some(&String::from("baz2")));

        let baz_command = exec_configs.commands.get("baz").unwrap().clone().commands.expect("baz command exists");
        assert_eq!(baz_command.len(), 3);
        assert_eq!(baz_command.get(0), Some(&String::from("bar3")));
        assert_eq!(baz_command.get(1), Some(&String::from("baz3")));
        assert_eq!(baz_command.get(2), Some(&String::from("flig3")));
    }
}