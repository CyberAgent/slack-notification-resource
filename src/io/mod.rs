use std::io::*;
use std::str::*;

fn read<T: FromStr>(s: &mut StdinLock) -> Option<T> {
    let s = s.by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok()
}
