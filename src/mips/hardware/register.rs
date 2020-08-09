/*
ReMIPS - A MARS-like MIPS Emulator
Copyright (C) 2020 Lewis Smith

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod Register {
    /// Register Object
    ///
    /// Consists of:
    /// * Register Name
    /// * Numeric Register Identifier
    /// * Current Register Value
    /// * Register Reset Value
    struct Register {
        name: String,
        number: i32,
        value: i32,
        reset_value: i32,
    }

    /// Register Constructor
    ///
    /// # Usage
    ///
    /// ```
    /// let mut new_register: Register = make_register("$s0", 16, 0, 0);
    /// ```
    fn make_register(reg_name: &String, reg_id: i32, reg_val: i32, reset_val: i32) -> Register {
        Register {
            name: reg_name,
            number: reg_id,
            value: reg_val,
            reset_value: reset_val,
        }
    }

    /// Register Instance Methods
    impl Register {
        /// Returns a register assembly name
        ///
        /// # Usage
        ///
        /// ```
        /// let reg_name: String = new_register.get_name();
        /// ````
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        /// Returns a register's numeric ID
        ///
        /// # Usage
        ///
        /// ```
        /// let reg_id: i32 = new_register.get_id();
        /// ```
        pub fn get_id(&self) -> i32 {
            self.number
        }

        /// Returns the value a register contains
        ///
        /// # Usage
        ///
        /// ```
        /// let reg_value = new_register.get_value();
        /// ```
        pub fn get_value(&self) -> i32 {
            self.value
        }

        /// Returns the default value of a register
        ///
        /// # Usage
        ///
        /// ```
        /// let reset_val = new_register.get_resetval();
        /// ```
        pub fn get_resetval(&self) -> i32 {
            self.reset_value
        }

        /// Sets the value of a register
        ///
        /// # Usage
        ///
        /// ```
        /// new_register.set_value(1234567);
        /// ```
        pub fn set_value(&mut self, new_val: i32) -> None {
            self.value = new_val
        }

        /// Sets the default value of a register
        ///
        /// # Usage
        ///
        /// ```
        /// new_register.set_resetval(1);
        /// ```
        pub fn set_resetval(&mut self, new_val: i32) -> None {
            self.reset_va;
            ie = new_val;
        }

        /// Resets the register value to its default value
        ///
        /// # Usage
        ///
        /// ```
        /// new_register.reset_register();
        pub fn reset_register(&mut self) -> None {
            self.value = self.reset_value;
        }
    }
}
