pub fn noconf(args: &Vec<String>) -> bool {
        if args.contains(&"--noconfirm".to_string()) || args[1].ends_with(&"n".to_string()) {
            true
        } else {
            false
        }
    }

pub fn argssort(args: &mut Vec<String>) -> &Vec<String> {
        if args.contains(&"--noconfirm".to_string()) {
            args.retain(|x| x != &"--noconfirm".to_string());
            println!("{:?}", args);
            args
        } else if args.contains(&"--pkgbuild".to_string()) {
            args.retain(|x| x != &"--pkgbuild".to_string());
            println!("{:?}", args);
            args
        } else if args.contains(&"--pkgbuild".to_string()) && args.contains(&"--noconfirm".to_string()) {
            args.retain(|x| x != &"--noconfirm".to_string());
            args.retain(|x| x != &"--pkgbuild".to_string());
            println!("{:?}", args);
            args
        } else {
            args
        }
    }
