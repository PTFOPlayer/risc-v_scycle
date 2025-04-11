use rust_hdl::prelude::*;

#[derive(LogicBlock)]
pub struct Alu {
    pub clock: Signal<In, Clock>,

    // simple sum
    pub alu_control: Signal<In, Bits<3>>,
    pub src1: Signal<In, Bits<32>>,
    pub src2: Signal<In, Bits<32>>,
    pub result: Signal<Out, Bits<32>>,
    pub zero: Signal<Out, Bit>,
}

impl Default for Alu {
    fn default() -> Self {
        Self {
            alu_control: Default::default(),
            src1: Default::default(),
            src2: Default::default(),
            result: Default::default(),
            zero: Default::default(),
            clock: Default::default(),
        }
    }
}

impl Logic for Alu {
    #[hdl_gen]
    fn update(&mut self) {
        if 0b000 == self.alu_control.val() {
            self.result.next = self.src1.val() + self.src2.val()
        } else {
            self.result.next = 0.into()
        }

        self.zero.next = self.result.val() == 0;
    }
}

#[cfg(test)]
mod alu_test {
    use rust_hdl::prelude::*;

    use crate::CLOCK_SPEED_HZ;

    use super::Alu;

    #[test]
    fn alu_test() {
        let mut sim = simple_sim!(Alu, clock, CLOCK_SPEED_HZ, ep, {
            let mut x = ep.init()?;

            // test sum
            x.alu_control.next = 0.into();
            x.src1.next = 10.into();
            x.src2.next = 20.into();

            wait_clock_cycles!(ep, clock, x, 1);

            assert_eq!(x.result.val(), 30);

            // clear
            x.src1.next = 0.into();
            x.src2.next = 0.into();
            wait_clock_cycles!(ep, clock, x, 1);

            // test default
            x.alu_control.next = 0b111.into();
            x.src1.next = 10.into();
            x.src2.next = 20.into();
            wait_clock_cycles!(ep, clock, x, 1);
            assert_eq!(x.result.val(), 0);

            ep.done(x)
        });
        let uut = Alu::default();

        sim.run_to_file(Box::new(uut), sim_time::ONE_SEC, "alu.vcd")
            .unwrap();
        vcd_to_svg(
            "alu.vcd",
            "alu_all.svg",
            &["uut.clock", "uut.zero", "uut.result"],
            0,
            sim_time::ONE_SEC,
        )
        .unwrap();
    }
}
