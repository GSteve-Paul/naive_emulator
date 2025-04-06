use naive_emulator::{
    common::PAddr,
    cpu::{cpu_gpr, cpu_run},
    loader::naive_load,
    memory::paddr_read,
};

fn loader() {
    let img = [
        // 0x8000_0000
        0b10000000000_00000_00000_00001_000101,
        0b00000000001_00000_00001_00001_000010,
        0b00000000000_00000_00001_00010_000110,
        0b11111111111_00010_00001_11100_000111,
        // 0x8000_0010
        0b00000000000_00000_00000_00000_000000,
        0b00000000000_00000_00000_00000_000000,
        0b00000000000_00000_00000_00000_000000,
        0b00000000000_00000_00000_00000_000000,
        // 0x8000_0020
        0b00000000000_00000_00000_00000_000010,
        0b00000000000_00000_00000_00000_000000,
        0b00000000000_00000_00000_00000_000000,
        0b00000000000_00000_00000_00000_000000,
    ];
    naive_load(&img);
}

#[test]
fn test_lui_lw_sw() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(1), 0x8000_0020);
    assert_eq!(cpu_gpr(2), 2);
    assert_eq!(paddr_read(PAddr(0x8000_001C), 4), 2);
    assert_eq!(paddr_read(PAddr(0x8000_0020), 4), 2);
}
