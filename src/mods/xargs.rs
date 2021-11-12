pub fn noconf(args: &Vec<String>) -> bool { // noconfirm if user passed --noconfirm or added n to the end of the arg
    if args.contains(&"--noconfirm".to_string()) || args[1].ends_with(&"n".to_string()) {
        true
    } else {
        false
    }
}

pub fn argssort(args: &mut Vec<String>) -> &Vec<String> { // sort the args
    if args.contains(&"--noconfirm".to_string()) {
        args.retain(|x| x != &"--noconfirm".to_string());
        args
    } else {
        args
    }
}
