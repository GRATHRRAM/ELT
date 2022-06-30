use std::io;
mod cmd;
fn main() {
    let mut inp = String::new();
    let mut lst = String::new();

    println!("ELT-Easy Linux Terminal V.prot:1");
    println!("if you new type help\n");
    while true {
        inp = String::new();
        io::stdin().read_line(&mut inp).expect("\nCan't read!!!");
        if inp.contains('\n') {
         inp.pop();
        }

        if inp == "show.files"{
            cmd::showfiles();
        }
        else if inp == "make.file"{
            cmd::makefile();
        }
        else if inp == "make.directory"{
            cmd::makedir();
        }
        else if inp == "remove.directory"{
            cmd::removedir();
        }
        else if inp == "remove.file"{
            cmd::removefile();
        }
        else if inp == "math.add"{
            cmd::mathadd();
        }
        else if inp == "math.sub"{
            cmd::mathsub();
        }
        else if inp == "clear"{
           std::process::Command::new("clear").status().unwrap();
        }
        else if inp == "help"{
            cmd::help();
        }
        else if inp == "cdcdcdcdcdcdcd"{
            cmd::changedir();
        }
        else if inp == "exit" {
            break;
        }
        else{
            println!("\nThis Command not exist!!!\n");
        }
    }
}
