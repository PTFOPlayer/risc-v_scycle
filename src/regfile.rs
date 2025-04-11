use rust_hdl::prelude::*;

const REGS: usize = 32;

#[derive(LogicBlock)]
pub struct Regfile {
    pub clock: Signal<In, Clock>,
    pub rst_n: Signal<In, Bit>,

    // Read
    // addr1 & addr2 of regs to read
    pub address1: Signal<In, Bits<5>>,
    pub address2: Signal<In, Bits<5>>,
    pub read_data1: Signal<Out, Bits<32>>,
    pub read_data2: Signal<Out, Bits<32>>,

    //write
    pub write_enable: Signal<In, Bit>,
    // addr of reg to write
    pub address3: Signal<In, Bits<5>>,
    pub write_data: Signal<In, Bits<32>>,

    pub registers: Vec<Signal<Local, Bits<32>>>,
}

impl Default for Regfile {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            rst_n: Default::default(),
            write_data: Default::default(),
            write_enable: Default::default(),
            registers: (0..REGS).map(|_| Signal::default()).collect(),
            address1: Default::default(),
            address2: Default::default(),
            read_data1: Default::default(),
            read_data2: Default::default(),
            address3: Default::default(),
        }
    }
}

impl Logic for Regfile {
    #[hdl_gen]
    fn update(&mut self) {
        if self.rst_n.val() == false {
            for i in 0..REGS {
                self.registers[i].next = 0.into();
            }
        } else if self.write_enable.val() && self.address3.val() != 0 {
            self.registers[self.address3.val().to_u32() as usize].next = self.write_data.val();
        }

        self.read_data1.next = self.registers[self.address1.val().to_u32() as usize].val();
        self.read_data2.next = self.registers[self.address2.val().to_u32() as usize].val();
    }
}

#[cfg(test)]
mod reg_tests {
    use rust_hdl::prelude::*;
    use rand::{Rng, rng};
    use crate::CLOCK_SPEED_HZ;

    use super::Regfile;


    #[test]
    fn reg_test() {
        let mut sim = simple_sim!(Regfile, clock, CLOCK_SPEED_HZ, ep, {
            let mut x = ep.init()?;

            x.rst_n.next = false;
            x.write_enable.next = false;
            x.address1.next = 0.into();
            x.address2.next = 0.into();
            x.address3.next = 0.into();
            x.read_data1.next = 0.into();
            x.read_data2.next = 0.into();
            x.write_data.next = 0.into();

            wait_clock_cycles!(ep, clock, x, 1);
            x.rst_n.next = true;
            wait_clock_cycles!(ep, clock, x, 1);

            for entry in x.registers.clone() {
                assert_eq!(entry.val(), 0);
                wait_clock_cycles!(ep, clock, x, 1);
            }

            let mut rng = rng();
            for _ in 0..500 {
                let addr1 = rng.random_range(1..31);
                let addr2 = rng.random_range(1..31);
                let addr3 = rng.random_range(1..31);
                let write_val = rng.random_range(0..0xFFFFFFFF);

                // read test from random reg
                x.address1.next = addr1.into();
                x.address2.next = addr2.into();
                wait_clock_cycles!(ep, clock, x, 1);

                assert_eq!(x.read_data1.val(), x.registers[addr1 as usize].val());
                assert_eq!(x.read_data2.val(), x.registers[addr2 as usize].val());

                // write test to random reg
                x.address3.next = addr3.into();
                wait_clock_cycles!(ep, clock, x, 1);
                x.write_enable.next = true;
                x.write_data.next = write_val.into();
                wait_clock_cycles!(ep, clock, x, 1);
                x.write_enable.next = false;

                assert_eq!(write_val, x.registers[addr3 as usize].val());
            }

            // zero reg test
            x.address3.next = 0.into();
            wait_clock_cycles!(ep, clock, x, 1);
            x.write_enable.next = true;
            x.write_data.next = 0xffff.into();
            wait_clock_cycles!(ep, clock, x, 1);
            x.write_enable.next = false;

            assert_eq!(0, x.registers[0].val());

            ep.done(x)
        });
        let uut = Regfile::default();

        sim.run_to_file(Box::new(uut), 3 * sim_time::ONE_SEC, "reg.vcd")
            .unwrap();
        vcd_to_svg(
            "reg.vcd",
            "reg_all.svg",
            &[
                "uut.clock",
                "uut.rst_n",
                "uut.write_enable",
                "uut.write_data",
            ],
            0,
            3 * sim_time::ONE_SEC,
        )
        .unwrap();
    }
}
