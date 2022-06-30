use std::fs;
use std::io;
use std::fs::File;
use std::io::prelude::*;
pub fn help(){
	print!("\ncomand list\n");
	print!("\nshow.files");
	print!("\nmake.file");
	print!("\nmake.directory");
	print!("\nremove.directory");
	print!("\nremove.file");
	print!("\nmath.add");
	print!("\nmath.sub");
	print!("\nclear");
	print!("\nexit\n");
}
pub fn showfiles(){
let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    print!("\n");
}
pub fn makefile(){
	let mut fln = String::new();
	println!("Type name of new file");
	io::stdin().read_line(&mut fln).expect("\nCan't read!!!");
        if fln.contains('\n') {
         	fln.pop();
        }
	let mut file = File::create(fln).expect("\nCan't create a file!!!");
	println!("done!\n");
}
pub fn makedir()
{
	let mut dir = String::new();
	println!("Type name of new directory");
	io::stdin().read_line(&mut dir).expect("\nCan't read!!!");
        if dir.contains('\n') {
         	dir.pop();
        }
	 fs::create_dir_all(dir).expect("\nCan't create a directory!!!");
	 println!("done!\n");
}
pub fn removedir(){
	let mut dir = String::new();
	dir = dir + "./";
	println!("Type name of directory");
	io::stdin().read_line(&mut dir).expect("\nCan't read!!!");
        if dir.contains('\n') {
         	dir.pop();
        }
	fs::remove_dir_all(dir).expect("No directory!!!");
}
pub fn removefile(){
	let mut ff = String::new();
	ff = ff + "./";
	println!("Type name of file");
	io::stdin().read_line(&mut ff).expect("\nCan't read!!!");
        if ff.contains('\n') {
         	ff.pop();
        }
	fs::remove_file(ff).expect("No file!!!");
}
pub fn mathadd(){
	let mut one = String::new();
	let mut two = String::new();
	println!("Type first number to add");
	io::stdin().read_line(&mut one).expect("\nCan't read!!!");
        if one.contains('\n') {
         	one.pop();
        }
    print!("Type second number to add\n");
    io::stdin().read_line(&mut two).expect("\nCan't read!!!");
        if two.contains('\n') {
         	two.pop();
        }
    let nm = one.parse::<i64>().expect("\nSome thing gone wrong!!!\n");
    let mn = two.parse::<i64>().expect("\nSome thing gone wrong!!!\n");

    println!("{} + {} = {}",one,two,nm + mn);
}
pub fn mathsub()
{
	let mut one = String::new();
	let mut two = String::new();
	println!("Type first number");
	io::stdin().read_line(&mut one).expect("\nCan't read!!!");
        if one.contains('\n') {
         	one.pop();
        }
    print!("Type second number\n");
    io::stdin().read_line(&mut two).expect("\nCan't read!!!");
        if two.contains('\n') {
         	two.pop();
        }
    let nm = one.parse::<i64>().expect("\nSome thing gone wrong!!!\n");
    let mn = two.parse::<i64>().expect("\nSome thing gone wrong!!!\n");

    println!("{} + {} = {}",one,two,nm - mn);
}
pub fn changedir(){
	let mut cd = String::new();
	cd = cd + "cd ";
	io::stdin().read_line(&mut cd).expect("\nCan't read!!!");
        if cd.contains('\n') {
         	cd.pop();
        }
	std::process::Command::new(cd).status().unwrap();
}