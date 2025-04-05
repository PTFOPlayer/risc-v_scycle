use rust_hdl::prelude::*;

use crate::WORDS;

#[derive(LogicBlock)]
pub struct Memory {
    pub clock: Signal<In, Clock>,
    pub rst_n: Signal<In, Bit>,

    pub address: Signal<In, Bits<32>>,
    pub write_data: Signal<In, Bits<32>>,
    pub write_enable: Signal<In, Bit>,

    pub read_data: Signal<Out, Bits<32>>,

    pub mem: Vec<Signal<Local, Bits<32>>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            rst_n: Default::default(),
            address: Default::default(),
            write_data: Default::default(),
            write_enable: Default::default(),
            read_data: Default::default(),
            mem: (0..WORDS).map(|_| Signal::default()).collect(),
        }
    }
}

impl Logic for Memory {
    #[hdl_gen]
    fn update(&mut self) {
        if self.rst_n.val() == false {
            for i in 0..self.mem.len() {
                self.mem[i].next = 0.into();
            }
        } else if self.write_enable.val() {
            if self.address.val().get_bits::<2>(0) == 0 {
                self.mem[self.address.val().get_bits::<30>(2).to_u32() as usize].next =
                    self.write_data.val();
            }
        }

        self.read_data.next =
            self.mem[self.address.val().get_bits::<30>(2).to_u32() as usize].val();
    }
}

#[cfg(test)]
mod mem_test {
    use rust_hdl::prelude::*;

    use crate::CLOCK_SPEED_HZ;

    use super::Memory;

    #[test]
    fn mem_test() {
        let mut sim = simple_sim!(Memory, clock, CLOCK_SPEED_HZ, ep, {
            let mut x = ep.init()?;

            x.rst_n.next = false;
            x.write_enable.next = false;
            x.address.next = 0.into();
            x.write_data.next = 0.into();

            wait_clock_cycles!(ep, clock, x, 1);
            x.rst_n.next = true;
            wait_clock_cycles!(ep, clock, x, 1);

            for entry in x.mem.clone() {
                assert_eq!(entry.val(), 0);
                wait_clock_cycles!(ep, clock, x, 1);
            }

            // test writing data onto addresses
            let test = [
                (0, 0xdeadbeef),
                (4, 0xcafebabe),
                (8, 0x1badb002),
                (8, 0x1baaaaad),
            ];

            for (addr, val) in test {
                x.address.next = addr.into();
                x.write_data.next = val.into();
                x.write_enable.next = true;
                wait_clock_cycles!(ep, clock, x, 48);

                x.write_enable.next = false;

                x.address.next = addr.into();
                wait_clock_cycles!(ep, clock, x, 48);
                assert_eq!(x.read_data.val(), val);
            }
            ep.done(x)
        });
        let uut = Memory::default();

        sim.run_to_file(Box::new(uut), sim_time::ONE_SEC, "mem.vcd")
            .unwrap();
        vcd_to_svg(
            "mem.vcd",
            "mem_all.svg",
            &[
                "uut.clock",
                "uut.rst_n",
                "uut.write_enable",
                "uut.write_data",
            ],
            0,
            sim_time::ONE_SEC,
        )
        .unwrap();
    }
}
