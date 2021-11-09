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
        args
    } else {
        args
    }
}
