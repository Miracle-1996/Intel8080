use crate::cpu::CPU;

impl CPU {
    pub fn export_snapshot(&mut self) -> Vec<u8> {
        let mut snapshot = Vec::new();

        // accumulator + 3 register pairs
        snapshot.push(self.reg.a);
        snapshot.push(self.reg.b);
        snapshot.push(self.reg.c);
        snapshot.push(self.reg.d);
        snapshot.push(self.reg.e);
        snapshot.push(self.reg.h);
        snapshot.push(self.reg.l);

        // Flags
        snapshot.push(self.flags.as_byte());

        // pc
        snapshot.extend_from_slice(&self.pc.to_be_bytes());

        // sp
        snapshot.extend_from_slice(&self.sp.to_be_bytes());

        // int
        snapshot.push((self.int.0) as u8);
        snapshot.push(self.int.1);

        // inte
        snapshot.push((self.inte) as u8);

        // Inserting a null byte
        snapshot.push(0);

        // slice_duration
        snapshot.extend_from_slice(&self.slice_duration.to_be_bytes());

        // slice_max_cycles
        snapshot.extend_from_slice(&self.slice_max_cycles.to_be_bytes());

        snapshot
    }
}
