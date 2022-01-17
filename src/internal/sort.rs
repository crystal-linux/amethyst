use crate::internal::{structs, rpc};

pub fn sort(a: &[String]) -> structs::Sorted {
    #[allow(unused_mut)]
    let mut repo: Vec<String> = vec![];
    let mut aur: Vec<String> = vec![];
    let mut nf: Vec<String> = vec![];

    for b in a {
        if rpc::rpcinfo(b.to_string()).found {
            aur.push(b.to_string());
        } else {
            nf.push(b.to_string());
        }
    }

    structs::Sorted::new(
        repo,
        aur,
        nf
    )
}