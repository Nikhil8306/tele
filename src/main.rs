mod dircontrol;
mod teleport;

fn main(){
    let args = std::env::args();
    if args.count() == 1 {
        println!("Navigation is not complete yet ...");
    }

    else {
        teleport::main();
    }

}   