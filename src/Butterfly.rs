use std::task;
use std::rt::io::timer::sleep;

///<Summary>
///Core how to Fly function
///</Summary>
#[inline]
fn fly<U>(animation: &[&str], symbols: int, delay: u64, f: &fn() -> U) -> U {
    let howtofly = animation.map(|x|x.to_owned());
    let (port, chan) = stream();
    do task::spawn_sched(task::SingleThreaded) {
        let mut prefix = ~"";
        for _ in range (0, symbols) {
            print(" ");
            prefix = format!("{:s}\x08", prefix);
        }
        while !port.peek() {
            for fly in howtofly.iter() {
                print!("{:s}{:s}", prefix, *fly);
                sleep(delay);
            };
        }
    }
    let ret = f();
    chan.send(());
    ret
}

///<Summary>
///Simple butterfly
///</Summary>
pub fn butterfly<U>(f: &fn() -> U) -> U {
    let animation = [&"|","/","-","\\"];
    fly(animation, 1, 1, f)
}

///<Summary>
///Stupid butterfly
///</Summary>
pub fn rustbuildbotdance<U>(f: &fn() -> U) -> U {
    let animation = [
        &"<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"];
    fly(animation, 7, 2, f)
}