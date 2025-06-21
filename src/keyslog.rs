pub fn get_keyboard_map() -> std::collections::HashMap<u8, u8> {
    let mut keyboard = std::collections::HashMap::new();
    keyboard.insert(0, 0);
    keyboard.insert(1, 0);
    keyboard.insert(2, 0);
    keyboard.insert(3, 0);
    keyboard.insert(4, 0);
    keyboard.insert(5, 0);
    keyboard.insert(6, 0);
    keyboard.insert(7, 0);
    keyboard.insert(8, 0);
    keyboard.insert(9, 0);
    keyboard.insert(0xA, 0);
    keyboard.insert(0xB, 0);
    keyboard.insert(0xC, 0);
    keyboard.insert(0xD, 0);
    keyboard.insert(0xE, 0);
    keyboard.insert(0xF, 0);
    return keyboard;
}
