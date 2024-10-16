// 8 bit CPU
// DO I do lines?


pub struct CPU {
    reg_a: u8, // accumulator

    reg_b: u8,
    reg_c: u8,

    reg_d: u8,
    reg_e: u8,

    reg_h: u8, // hl reg is cheater reg
    reg_l: u8,


    stack_ptr: u16,
    program_counter: u16,


    address_bus: u32,
    ir: u32,

    zero_flag: bool,
    subtract_flag: bool,
    half_carry_flag: bool,
    carry_flag: bool

    // speed 2.10 MHz in speedy mode 1.05 HZ otherwise ( period of )
}


impl CPU {

}


