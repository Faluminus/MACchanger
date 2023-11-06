use std::io;
use rand::{Rng, thread_rng as rng};
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
use winreg::RegKey;
use system_shutdown::reboot;


fn main() -> io::Result<()> {
   find_and_set_register()?;
   reboot().unwrap();
   Ok(())

}

fn get_random_mac() -> String {
   let mut mac = String::new();
   for _ in 0..6 {
      let a = rng().gen_range(0..256);
      mac += &format!("{:02X}", a);
   }
   mac
}

fn find_and_set_register() -> io::Result<()> {
   let x = get_random_mac();
   let reg = RegKey::predef(HKEY_LOCAL_MACHINE);
   let findzzone = reg.open_subkey_with_flags(
      r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}\0001",
      KEY_WRITE,
   )?;

   if let Ok(value) = findzzone.get_value::<String, _>("NetworkAddress") {
      findzzone.set_value("NetworkAddress", &x)?;
   } else {
      findzzone.create_subkey("NetworkAddress")?;
      findzzone.set_value("NetworkAddress", &x)?;
   }

   Ok(())
}


