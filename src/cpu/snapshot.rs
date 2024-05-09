use crate::{cpu::CPU, bus::SnapshotError};

impl CPU {
    pub fn export_snapshot(&mut self) -> Vec<u8> {
        let mut snapshot = Vec::new();

        // Magic number
        snapshot.extend_from_slice(&[0x41, 0x4c, 0x54, 0x52]);

        // Snapshot format version + 3 null bytes
        snapshot.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

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

        // 12 null bytes
        snapshot.extend_from_slice(&[0x00; 12]);

        // ROM start / end
        let r = self.bus.get_romspace();
        snapshot.extend_from_slice(r.0.to_be_bytes().as_slice());
        snapshot.extend_from_slice(r.1.to_be_bytes().as_slice());

        // RAM
        snapshot.extend_from_slice(self.bus.export_address_space().as_slice());

        snapshot
    }

    pub fn import_snapshot(&mut self, snapshot: Vec<u8>) -> Result<(), SnapshotError> {
        if snapshot[0..=3] != [0x41, 0x4c, 0x54, 0x52] {
            return Err(SnapshotError::InvalidHeader);
        }

        // CPU registers
        self.reg.a = snapshot[0x08];
        self.reg.b = snapshot[0x09];
        self.reg.c = snapshot[0x0A];
        self.reg.d = snapshot[0x0B];
        self.reg.e = snapshot[0x0C];
        self.reg.h = snapshot[0x0D];
        self.reg.l = snapshot[0x0E];

        // CPU flags
        self.flags.from_byte(snapshot[0x0F]);

        // PC
        self.pc = u16::from_be_bytes([snapshot[0x10], snapshot[0x11]]);

        // SP
        self.sp = u16::from_be_bytes([snapshot[0x12], snapshot[0x13]]);

        // int
        self.int = (snapshot[0x14] != 0, snapshot[0x15]);

        // inte
        self.inte = snapshot[0x16] != 0;

        // slice_duration
        let slice_duration = u32::from_be_bytes([
            snapshot[0x18],
            snapshot[0x19],
            snapshot[0x1A],
            snapshot[0x1B],
        ]);
        self.slice_duration = slice_duration;

        // slice_max_cycles
        let slice_max_cycles = u32::from_be_bytes([
            snapshot[0x1C],
            snapshot[0x1D],
            snapshot[0x1E],
            snapshot[0x1F],
        ]);
        self.slice_max_cycles = slice_max_cycles;

        // Address space
        self.bus
            .load_from_vec(snapshot[0x30..snapshot.len()].to_vec(), 0)?;

        Ok(())
    }
}
