fn main( ) {
   use sysinfo::{ProcessExt, SystemExt, ComponentExt};

   let mut system = sysinfo::System::new_all();

   // First we update all information of our system struct.
   system.refresh_all();

   // // Now let's print every process' id and name:
   // for (pid, proc_) in system.get_processes() {
   //    println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
   // }

   // Then let's print the temperature of the different components:
   for component in system.get_components() {
      println!("{:?}", component.get_label());
   }

   // And then all disks' information:
   for disk in system.get_disks() {
      println!("{:?}", disk);
   }

   // And finally the RAM and SWAP information:
   println!("total memory: {} KB", system.get_total_memory());
   println!("used memory : {} KB", system.get_used_memory());
   println!("total swap  : {} KB", system.get_total_swap());
   println!("used swap   : {} KB", system.get_used_swap());

   // Display system information:
   println!("System name:      {:?}", system.get_name());
   println!("System version:   {:?}", system.get_version());
   println!("System host name: {:?}", system.get_host_name());
}