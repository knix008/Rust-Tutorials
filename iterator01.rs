fn main(){
   let msg = "Tutorials Point has good tutorials".to_string();
   let mut i = 1;
   
   for token in msg.split_whitespace(){
      println!("token {} {}",i,token);
      i+=1;
   }
}

