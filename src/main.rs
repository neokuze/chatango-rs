//#[allow(unused_imports)]
mod room;


fn main() {
   println!("{:?}",room::get_server("grouphelp")); // server s31
   println!("{:?}",room::get_server("animeotakuchatroom")); // server s35
   println!("{:?}",room::get_server("thegraygarden")); // server s9
}