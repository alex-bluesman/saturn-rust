// Copyright (c) 2025 Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//
// Licensed under the MIT License (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
// http://opensource.org/licenses/MIT
//
// Unless required by applicable law or agreed to in writing, software distributed 
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR 
// CONDITIONS OF ANY KIND, either express or implied. See the License for the 
// specific language governing permissions and limitations under the License.

const PL011_REG_BASE: usize = 0x0900_0000;
const PL011_REG_DR: usize = 0x00;

static LOGO: &[u8] = b"start kernel...\r\n";

pub(crate) fn init() -> ! {
    let dr = (PL011_REG_BASE + PL011_REG_DR) as *mut u8;

    for c in LOGO {
        unsafe {
            dr.write_volatile(*c);
        }
    }

    loop {}
}
