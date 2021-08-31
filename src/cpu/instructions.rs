use crate::bus_objects::BusObject;
use crate::cpu;
#[derive(Clone, Copy, Debug)]
pub struct OpDecodedInstruction {
    pub op: u8,
    other: u32,
}

impl OpDecodedInstruction {
    pub fn decode(i: u32) -> Self {
        OpDecodedInstruction {
            op: ((i & 0xFC000000) >> 26) as u8,
            other: i,
        }
    }
}

pub trait FromOpDecodedInstruction {
    fn decode(i: OpDecodedInstruction) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub struct ITypeInstruction {
    pub op: u8,
    rs: u8,
    pub rt: u8, //for regimm instruction function field
    immediate: u16,
}

impl ITypeInstruction {
    #[inline]
    pub fn rs(&self) -> u8 {
        self.rs
    }

    #[inline]
    pub fn rt(&self) -> u8 {
        self.rt
    }
    #[inline]
    pub fn immediate(&self) -> u16 {
        self.immediate
    }
}

impl FromOpDecodedInstruction for ITypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        let im = i.other & 0x0000FFFF;
        let rt = (i.other & 0x001F0000) >> 16;
        let rs = (i.other & 0x03E00000) >> 21;
        ITypeInstruction {
            op: i.op,
            rs: rs as u8,
            rt: rt as u8,
            immediate: im as u16,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JTypeInstruction {
    pub op: u8,
    target: u32,
}

impl JTypeInstruction {
    #[inline]
    pub fn target(&self) -> u32 {
        self.target
    }
}

impl FromOpDecodedInstruction for JTypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        JTypeInstruction {
            op: i.op,
            target: i.other & 0x03FFFFFF,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RTypeInstruction {
    pub op: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    pub funct: u8,
}

impl RTypeInstruction {
    #[inline]
    pub fn rs(&self) -> u8 {
        self.rs
    }
    #[inline]
    pub fn rt(&self) -> u8 {
        self.rt
    }
    #[inline]
    pub fn rd(&self) -> u8 {
        self.rd
    }
}

impl FromOpDecodedInstruction for RTypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        let rs = (i.other & 0x03E00000) >> 21;
        let rt = (i.other & 0x001F0000) >> 16;
        let rd = (i.other & 0x0000F800) >> 11;
        let shamt = (i.other & 0x000007C0) >> 6;
        let funct = i.other & 0x0000003F;
        RTypeInstruction {
            op: i.op,
            rs: rs as u8,
            rt: rt as u8,
            rd: rd as u8,
            shamt: shamt as u8,
            funct: funct as u8,
        }
    }
}
#[inline]
fn generic_branch(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction, cond: bool) {
    let offset = (i.immediate as i16 as i32) << 2;
    if cond {
        cpu.branch = true;
        cpu.branch_target = ((cpu.pc + 4) as i32 + offset) as u32;
    }
}

//add 2 integers
//throw Integer overflow exception on overflow
pub fn add(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    //no exception
    addu(cpu, i);
}
//add content of one register to immediate sign extended value
//throw Integer overflow exception on overflow
pub fn addi(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    //no exception
    addiu(cpu, i);
}
//add content of one register to immediate sign extended value
pub fn addiu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = cpu.get_register(i.rs) as i32;
    let imm = i.immediate as i16 as i32;
    cpu.set_register(i.rt, (x + imm) as u32);
}
//add 2 signed integers
pub fn addu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.set_register(i.rd, ((x as i32) + (y as i32)) as u32);
}

pub fn and(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.set_register(i.rd, x & y);
}

pub fn andi(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = cpu.get_register(i.rs);
    let imm = i.immediate as u32;
    cpu.set_register(i.rt, x & imm);
}

pub fn beq(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, cpu.get_register(i.rs) == cpu.get_register(i.rt))
}

pub fn bgez(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, cpu.get_register(i.rs) as i32 >= 0)
}

pub fn bgezal(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    bgez(cpu, i);
    cpu.set_register(31, cpu.pc + 8);
}

pub fn bgtz(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, cpu.get_register(i.rs) as i32 > 0)
}

pub fn blez(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, cpu.get_register(i.rs) as i32 <= 0)
}

pub fn bltz(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, (cpu.get_register(i.rs) as i32) < 0)
}

pub fn bltzal(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    bltz(cpu, i);
    cpu.set_register(31, cpu.pc + 8);
}

pub fn bne(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    generic_branch(cpu, i, cpu.get_register(i.rs) != cpu.get_register(i.rt))
}

pub fn div(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs) as i32;
    let y = cpu.get_register(i.rt) as i32;
    cpu.lo = (x / y) as u32;
    cpu.hi = (x % y) as u32;
}

pub fn divu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.lo = x / y;
    cpu.hi = x % y;
}

pub fn j(cpu: &mut cpu::MipsCpu<'_>, i: JTypeInstruction) {
    let target = (i.target << 2) | (0xF0000000 & cpu.pc);
    cpu.branch = true;
    cpu.branch_target = target;
}

pub fn jal(cpu: &mut cpu::MipsCpu<'_>, i: JTypeInstruction) {
    j(cpu, i);
    cpu.set_register(31, cpu.pc + 8);
}

pub fn jalr(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    jr(cpu, i);
    cpu.set_register(i.rd, cpu.pc + 8);
}

pub fn jr(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    cpu.branch = true;
    cpu.branch_target = cpu.get_register(i.rs);
}

pub fn lb(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    let loaded = cpu.bus.read_byte(addr as u32) as i8 as i32;
    cpu.set_register(i.rt, loaded as u32);
}

pub fn lbu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    let loaded = cpu.bus.read_byte(addr as u32) as u32;
    cpu.set_register(i.rt, loaded);
}

pub fn lh(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    let loaded = u16::from_be(cpu.bus.read_hw(addr as u32)) as i16 as i32;
    cpu.set_register(i.rt, loaded as u32);
}

pub fn lhu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    let loaded = u16::from_be(cpu.bus.read_hw(addr as u32)) as u32;
    cpu.set_register(i.rt, loaded);
}

pub fn lui(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let to_load = (i.immediate as u32) << 16;
    cpu.set_register(i.rt, to_load);
}

pub fn lw(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    let loaded = cpu.bus.read_w(addr as u32);
    cpu.set_register_nc(i.rt, loaded);
}

pub fn lwl(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let mut addr = ((i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32) as u32;
    let mut register = cpu.get_register_nc(i.rt);
    let mut counter = 0;
    while addr % 4 != 0 {
        register &= 0xff << ((3 - counter) * 8);
        register |= (cpu.bus.read_byte(addr) as u32) << ((3 - counter) * 8);
        counter += 1;
        addr += 1;
    }
    cpu.set_register_nc(i.rt, register);
}

pub fn lwr(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let mut addr = ((i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32) as u32;
    let mut register = cpu.get_register_nc(i.rt);
    let mut counter = 0;
    while addr % 4 != 0 {
        register &= 0xff << (counter * 8);
        register |= (cpu.bus.read_byte(addr) as u32) << (counter * 8);
        counter += 1;
        addr -= 1;
    }
    cpu.set_register_nc(i.rt, register);
}

pub fn mfhi(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    cpu.set_register(i.rd, cpu.hi);
}

pub fn mflo(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    cpu.set_register(i.rd, cpu.lo);
}

pub fn mthi(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    cpu.hi = cpu.get_register(i.rs);
}

pub fn mtlo(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    cpu.lo = cpu.get_register(i.rs);
}

pub fn mult(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs) as i32 as i64;
    let y = cpu.get_register(i.rt) as i32 as i64;
    let r = x * y;
    cpu.lo = r as u32;
    cpu.hi = (r >> 32) as u32;
}

pub fn multu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs) as u64;
    let y = cpu.get_register(i.rt) as u64;
    let r = x * y;
    cpu.lo = r as u32;
    cpu.hi = (r >> 32) as u32;
}

pub fn nor(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.set_register(i.rd, !(x | y));
}

pub fn or(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.set_register(i.rd, x | y);
}

pub fn ori(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = cpu.get_register(i.rs);
    let imm = i.immediate as u32;
    cpu.set_register(i.rt, x | imm);
}

pub fn sb(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    cpu.bus
        .write_byte(addr as u32, cpu.get_register(i.rt) as u8);
}

pub fn sh(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    cpu.bus
        .write_hw(addr as u32, u16::to_be(cpu.get_register(i.rt) as u16));
}

pub fn sll(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    cpu.set_register(i.rd, x << i.shamt);
}

pub fn sllv(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    let y = cpu.get_register(i.rs);
    cpu.set_register(i.rd, x << y);
}

pub fn slt(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt) as i32;
    let y = cpu.get_register(i.rs) as i32;
    if y < x {
        cpu.set_register(i.rd, 1);
    } else {
        cpu.set_register(i.rd, 0);
    }
}

pub fn slti(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = i.immediate as i16 as i32;
    let y = cpu.get_register(i.rs) as i32;
    if y < x {
        cpu.set_register(i.rt, 1);
    } else {
        cpu.set_register(i.rt, 0);
    }
}

pub fn sltiu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = i.immediate as i16 as u32;
    let y = cpu.get_register(i.rs);
    if y < x {
        cpu.set_register(i.rt, 1);
    } else {
        cpu.set_register(i.rt, 0);
    }
}

pub fn sltu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    let y = cpu.get_register(i.rs);
    if y < x {
        cpu.set_register(i.rd, 1);
    } else {
        cpu.set_register(i.rd, 0);
    }
}

pub fn sra(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt) as i32;
    cpu.set_register(i.rd, (x >> i.shamt) as u32);
}

pub fn srav(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt) as i32;
    let y = cpu.get_register(i.rs);
    cpu.set_register(i.rd, (x >> y) as u32);
}

pub fn srl(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    cpu.set_register(i.rd, x >> i.shamt);
}

pub fn srlv(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    let y = cpu.get_register(i.rs);
    cpu.set_register(i.rd, x >> y);
}

pub fn sub(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    let y = cpu.get_register(i.rs);
    cpu.set_register(i.rd, y - x);
}

pub fn subu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    sub(cpu, i);
}

pub fn sw(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let addr = (i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32;
    cpu.bus.write_w(addr as u32, cpu.get_register_nc(i.rt));
}

pub fn swl(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let mut addr = ((i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32) as u32;
    let register = cpu.get_register_nc(i.rt);
    let mut counter = 0;
    while addr % 4 != 0 {
        let value = (register >> ((3 - counter) * 8)) as u8;
        cpu.bus.write_byte(addr, value);
        counter += 1;
        addr += 1;
    }
}

pub fn swr(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let mut addr = ((i.immediate as i16 as i32) + cpu.get_register(i.rs) as i32) as u32;
    let register = cpu.get_register_nc(i.rt);
    let mut counter = 0;
    while addr % 4 != 0 {
        let value = (register >> (counter * 8)) as u8;
        cpu.bus.write_byte(addr, value);
        counter += 1;
        addr -= 1;
    }
}

pub fn xor(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rt);
    let y = cpu.get_register(i.rs);
    cpu.set_register(i.rd, x ^ y);
}

pub fn xori(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = cpu.get_register(i.rs);
    let imm = i.immediate as u32;
    cpu.set_register(i.rt, x ^ imm);
}
