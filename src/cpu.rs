pub struct Reg8 {
    pub value: u8
}

pub struct Bytes {
    pub hi: u8,
    pub lo: u8
}

pub union Reg16 {
    pub value: u16,
    pub bytes: std::mem::ManuallyDrop<Bytes>
}

pub struct  REGS {
    pub v0: Reg8,
    pub v1: Reg8,
    pub v2: Reg8,
    pub v3: Reg8,
    pub v4: Reg8,
    pub v5: Reg8,
    pub v6: Reg8,
    pub v7: Reg8,
    pub v8: Reg8,
    pub v9: Reg8,
    pub va: Reg8,
    pub vb: Reg8,
    pub vc: Reg8,
    pub vd: Reg8,
    pub ve: Reg8,
    pub vf: Reg8,
    pub i: Reg16,
}

pub struct CPU {
    pub pc: u16,
    pub sp: Vec<u16>,
    pub regs: REGS,
    pub dt: u8,
    pub st: u8
}

pub fn get_cpu() -> CPU {
    let init_regs = REGS {
        v0: Reg8 {value: 0},
        v1: Reg8 {value: 0},
        v2: Reg8 {value: 0},
        v3: Reg8 {value: 0},
        v4: Reg8 {value: 0},
        v5: Reg8 {value: 0},
        v6: Reg8 {value: 0},
        v7: Reg8 {value: 0},
        v8: Reg8 {value: 0},
        v9: Reg8 {value: 0},
        va: Reg8 {value: 0},
        vb: Reg8 {value: 0},
        vc: Reg8 {value: 0},
        vd: Reg8 {value: 0},
        ve: Reg8 {value: 0},
        vf: Reg8 {value: 0},
        i: Reg16 {value: 0}
    };

    CPU { pc: 0x200, sp: vec!(), regs: init_regs, dt: 0, st: 0 }
}

pub fn get_reg_val(cpu: &mut CPU, reg_byte: u8) -> u8 {
    match reg_byte {
        0x0 => return cpu.regs.v0.value,
        0x1 => return cpu.regs.v1.value,
        0x2 => return cpu.regs.v2.value,
        0x3 => return cpu.regs.v3.value,
        0x4 => return cpu.regs.v4.value,
        0x5 => return cpu.regs.v5.value,
        0x6 => return cpu.regs.v6.value,
        0x7 => return cpu.regs.v7.value,
        0x8 => return cpu.regs.v8.value,
        0x9 => return cpu.regs.v9.value,
        0xA => return cpu.regs.va.value,
        0xB => return cpu.regs.vb.value,
        0xC => return cpu.regs.vc.value,
        0xD => return cpu.regs.vd.value,
        0xE => return cpu.regs.ve.value,
        0xF => return cpu.regs.vf.value,
        _ => panic!("Invalid register index: {}", reg_byte),
    }
}

pub fn set_reg_val(cpu: &mut CPU, val: u8, reg_byte: u8) {
    match reg_byte {
        0x0 => cpu.regs.v0.value = val,
        0x1 => cpu.regs.v1.value = val,
        0x2 => cpu.regs.v2.value = val,
        0x3 => cpu.regs.v3.value = val,
        0x4 => cpu.regs.v4.value = val,
        0x5 => cpu.regs.v5.value = val,
        0x6 => cpu.regs.v6.value = val,
        0x7 => cpu.regs.v7.value = val,
        0x8 => cpu.regs.v8.value = val,
        0x9 => cpu.regs.v9.value = val,
        0xA => cpu.regs.va.value = val,
        0xB => cpu.regs.vb.value = val,
        0xC => cpu.regs.vc.value = val,
        0xD => cpu.regs.vd.value = val,
        0xE => cpu.regs.ve.value = val,
        0xF => cpu.regs.vf.value = val,
        _ => panic!("Invalid register index: {}", reg_byte),
    }
}
