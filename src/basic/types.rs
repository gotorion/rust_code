#![allow(unused_variables)]
type File = String;

#[allow(dead_code)]
fn open(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!("not implemented yet");
}
