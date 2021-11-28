pub fn noconf(args: &[String]) -> bool {
    // noconfirm if user passed --noconfirm or added n to the end of the arg
    args.contains(&"--noconfirm".to_string()) || args[0].ends_with(&"n".to_string())
}

pub fn argssort(args: &mut Vec<String>) -> &Vec<String> {
    // sort the args
    if args.contains(&"--noconfirm".to_string()) {
        args.retain(|x| x != &"--noconfirm".to_string());
        return args;
    }
    args
}
