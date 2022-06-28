use crate::cpu::Cpu;

pub fn exec_opcode(cpu: &mut Cpu) -> u8 {
  let opcode = cpu.read_opcode();

  match opcode {
    
      0x18 => { 
  println!("JR r8 ");

          let delta = cpu.read_n() as i8; 


        cpu.jr(delta);



  12
},

    
      0x20 => { 
  println!("JR NZ r8");

          let delta = cpu.read_n() as i8; 

 if !cpu.reg.flag_get(Z) { 
        cpu.jr(delta);
 } 


  12
},

    
      0x28 => { 
  println!("JR Z r8");

          let delta = cpu.read_n() as i8; 

 if cpu.reg.flag_get(Z) { 
        cpu.jr(delta);
 } 


  12
},

    
      0x30 => { 
  println!("JR NC r8");

          let delta = cpu.read_n() as i8; 

 if !cpu.reg.flag_get(C) { 
        cpu.jr(delta);
 } 


  12
},

    
      0x38 => { 
  println!("JR C r8");

          let delta = cpu.read_n() as i8; 

 if cpu.reg.flag_get(C) { 
        cpu.jr(delta);
 } 


  12
},

    
  }
}
