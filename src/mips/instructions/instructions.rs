pub enum InstructionFormat {
    RFormat,
    IFormat,
    JFormat,
}

pub struct Register {
    name: String,
    number: i32,
    value: i32,
    reset_value: i32,
}

mod MIPS {
    use Register;
    /// # Null Operation
    ///
    /// Machine code is all zeroes
    /// # Format
    ///
    /// R-Format
    ///
    /// # opcode
    ///
    /// ```000000 00000 00000 00000 00000 000000```
    pub fn nop() -> Result<String, String> {
        Err("Null Operation Called")
    }
    /// # Addition with overflow
    ///
    /// # Usage
    /// ```add $t1, $t2. $t3``` - Sets $t1 to the sum of $t2 and $t3
    ///
    /// # Format
    ///
    /// R-Format
    ///
    /// # opcode
    ///
    /// ```000000 sssss ttttt fffff 00000 100000```
    pub fn add(reg_1: &mut Register, reg_2: &Register, reg_3: &Register) -> Result<(), String> {
        let add1: i32 = reg_2.value;
        let add2: i32 = reg_3.value;
        let sum = add1 + add2;
        if (add1 >= 0 && add2 >= 0 && sum < 0) || (add1 < 0 && add2 < 0 && sum >= 0) {
            Err("Arithmetic Overflow")
        }
        reg_3.value = sum;
        Ok(())
    }
    /// # Subtraction with overflow
    ///
    /// # Usage
    /// ```sub $t1, $t2. $t3``` - Sets $t1 to the difference of $t2 and $t3
    ///
    /// # Format
    ///
    /// R-Format
    ///
    /// # opcode
    ///
    /// ```000000 sssss ttttt fffff 00000 100010```
    pub fn sub(reg_1: &mut Register, reg_2: &Register, reg_3: &Register) -> Result<(), String> {
        let sub1: i32 = reg_2.value;
        let sub2: i32 = reg_3.value;
        let diff: i32 = sub1 - sub2;
        if (sub1 >= 0 && sub2 < 0 && dif < 0) || (sub1 < 0 && sub2 >= 0 && dif >= 0) {
            Err("Arithmetic Overflow")
        }
        reg_3.value = diff;
        Ok(())
    }
    /// # Addition immediate with overflow
    ///
    /// # Usage
    /// ```addi $t1, $t2. -100``` - Sets $t1 to the difference of $t2 and a signed 16-bit immediate value
    ///
    /// # Format
    ///
    /// I-Format
    ///
    /// # opcode
    ///
    /// ```001000 sssss fffff tttttttttttttttt```
    pub fn addi(reg_1: &mut Register, reg_2: &Register, i_val: i16) -> Result<(), String> {
        let add1: i32 = reg_2.value;
        let add2: i16 = i_val;
        let sum: i32 = add1 + add2;
        if (add1 >= 0 && add2 >= 0 && sum < 0) || (add1 < 0 && add2 < 0 && sum >= 0) {
            Err("Arithmetic Overflow")
        }
        reg_1.value = sum;
        Ok(())
    }
    /// # Addition unsigned without overflow
    ///
    /// # Usage
    /// ```addu $t1, $t2. $t3``` - Sets $t1 to the sum of $t2 and $t3, no overflow
    ///
    /// # Format
    ///
    /// R-Format
    ///
    /// # opcode
    ///
    /// ```000000 sssss ttttt fffff 00000 100001```
    pub fn addu(reg_1: &mut Register, reg_2: &Register, reg_3: &Register) -> Result<(), String> {
        let add1: u32 = reg_2.value as u32;
        let add2: u32 = reg_3.value as u32;
        if let Some(sum) = add1.checked_add(add2) {
            reg_1.value = sum;
            Ok(())
        } else {
            Err("Arithmetic Error")
        }
    }
    /// # Subtraction unsigned without overflow
    ///
    /// # Usage
    /// ```subu $t1, $t2. $t3``` - Sets $t1 to the difference of $t2 and $t3, no overflow
    ///
    /// # Format
    ///
    /// R-Format
    ///
    /// # opcode
    ///
    /// ```000000 sssss ttttt fffff 00000 100011```
    pub fn subu(reg_1: &mut Register, reg_2: &Register, reg_3: &Register) -> Result<(), String> {
        let sub1: u32 = reg_2.value as u32;
        let sub2: u32 = reg_2.value as u32;
        if let Some(diff) = sub1.checked_sub(sub2) {
            reg_1.value = diff;
            Ok(())
        } else {
            Err("Arithmetic Error")
        }
    }
    /// # Addition immediate unsigned without overflow
    ///
    /// # Usage
    /// ```subu $t1, $t2. $t3``` - Sets $t1 to the sum of $t2 and a signed 16-bit immediate, no overflow
    ///
    /// # Format
    ///
    /// I-Format
    ///
    /// # opcode
    ///
    /// ```001001 sssss fffff tttttttttttttttt```
    pub fn addiu(reg_1: &mut Register, reg_2: Register, i_val: i16) -> Result<(), String> {
        let add1: u32 = reg_2.value as u32;
        let add2: u16 = i_val as u16;
        if let Some(sum) = add1.checked_add(add2) {
            reg_1.value = sum;
            Ok(())
        } else {
            Err("Arithmetic Error")
        }
    }
}
