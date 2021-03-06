mod memory {
    enum ConfigTypes {
        Default,
        DataBasedCompact,
        TextBasedCompact,
    }

    fn set_memconfig(c_type: ConfigTypes) -> Vec<i32> {
        let config_array: [i32] = match c_type {
            Default => [
                0x00400000, // .text Base Address
                0x10000000, // Data Segment Base Address
                0x10000000, // .extern Base Address
                0x10008000, // Global Pointer ($gp)
                0x10010000, // .data Base Address
                0x10040000, // Heap Base Address
                0x7fffeffc, // Stack Pointer ($sp)
                0x7ffffffc, // Stack Base Address
                0x7fffffff, // Highest Address in Userspace
                0x80000000, // Lowest Address in Kernelspace
                0x80000000, // .ktext Base Address
                0x80000180, // Exception Handler Address
                0x90000000, // .kdata Base Address
                0xffff0000, // MMIO Base Address
                0xffffffff, // Highest Address in Kernel & Memory
                0x7fffffff, // Data Segment Limit Address
                0x0ffffffc, // Text Segment Limit Address
                0xfffeffff, // Kernel Data Segment Limit Address
                0x8ffffffc, // Kernel Text Limit Address
                0x10040000, // Stack Limit Address
                0xffffffff, // Memory Map Limit Address
            ],
            // For 16-Bit addressing, data segment starts at 0
            DataBasedCompact => [
                0x00003000, // .text Base Address
                0x00000000, // Data Segment base address
                0x00001000, // .extern Base Address
                0x00001800, // Global Pointer ($gp)
                0x00000000, // .data base Address
                0x00002000, // heap base address
                0x00002ffc, // stack pointer ($sp)
                0x00002ffc, // stack base address
                0x00003fff, // highest address in user space
                0x00004000, // lowest address in kernel space
                0x00004000, // .ktext base address
                0x00004180, // exception handler address
                0x00005000, // .kdata base address
                0x00007f00, // MMIO base address
                0x00007fff, // highest address in kernel (and memory)
                0x00002fff, // data segment limit address
                0x00003ffc, // text limit address
                0x00007eff, // kernel data segment limit address
                0x00004ffc, // kernel text limit address
                0x00002000, // stack limit address
                0x00007fff, // memory map limit address
            ],
            // For 16-bit addressing, text segment starts at 0
            TextBasedCompact => [
                0x00000000, // .text Base Address
                0x00001000, // Data Segment Base Address
                0x00001000, // .extern Base Address
                0x00001800, // Global Pointer ($gp)
                0x00002000, // .data Base Address
                0x00003000, // heap Base Address
                0x00003ffc, // Stack Pointer ($sp)
                0x00003ffc, // Stack Base Address
                0x00003fff, // Highest Address in Userspace
                0x00004000, // Lowest Address in Kernelspace
                0x00004000, // .ktext Base Address
                0x00004180, // Exception Handler Address
                0x00005000, // .kdata Base Address
                0x00007f00, // MMIO Base Address
                0x00007fff, // Highest Address in Kernel & Memory
                0x00003fff, // Data Segment Limit Address
                0x00000ffc, // Text Limit Address
                0x00007eff, // Kernel Data Segment Limit Address
                0x00004ffc, // Kernel Text Limit Address
                0x00003000, // Stack Limit Address
                0x00007fff, // Memory Map Limit Address
            ],
        };
        config_array.to_vec()
    }
}
