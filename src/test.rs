#[cfg(test)]
use crate::chip::Chip;

#[test]
fn test_memory_get_and_set() {
    let mut chip: Chip = Chip::new();
    chip.set_byte(0x200, 200);
    let testvar = chip.get_addr(0x200);
    assert_eq!(200, testvar);
}
