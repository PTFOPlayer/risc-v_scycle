use rust_hdl::prelude::*;

#[derive(LogicBlock)]
pub struct Control {
    pub clock: Signal<In, Clock>,

    pub op: Signal<In, Bits<7>>,
    // decoded function
    pub func3: Signal<In, Bits<3>>,
    pub func7: Signal<In, Bits<7>>,
    pub alu_zero: Signal<In, Bit>,

    // alu control section
    pub alu_control: Signal<Out, Bits<3>>,
    pub imm_source: Signal<Out, Bits<2>>,
    pub mem_write: Signal<Out, Bit>,
    pub reg_write: Signal<Out, Bit>,

    control_op: Signal<Local, Bits<2>>,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            op: Default::default(),
            func3: Default::default(),
            func7: Default::default(),
            alu_zero: Default::default(),
            alu_control: Default::default(),
            imm_source: Default::default(),
            mem_write: Default::default(),
            reg_write: Default::default(),
            control_op: Default::default(),
        }
    }
}

impl Logic for Control {
    #[hdl_gen]
    fn update(&mut self) {
        // decoder
        // lw
        if self.op.val() == 0b0000011 {
            self.reg_write.next = true;
            self.imm_source.next = 0b00.into();
            self.mem_write.next = false;
            self.control_op.next = 0b00.into();
        } else {
            self.reg_write.next = false;
            self.imm_source.next = 0b00.into();
            self.mem_write.next = false;
            self.control_op.next = 0b00.into();
        }

        // control decoder
        if self.control_op.val() == 0b00 {
            // lw sw

            self.alu_control.next = 0b000.into();
        } else {
            self.alu_control.next = 0b111.into();
        }
    }
}

#[cfg(test)]
mod control_test {
    use rust_hdl::prelude::*;

    use crate::CLOCK_SPEED_HZ;

    use super::Control;

    #[test]
    fn control_test() {
        let mut sim = simple_sim!(Control, clock, CLOCK_SPEED_HZ, ep, {
            let mut x = ep.init()?;

            wait_clock_cycles!(ep, clock, x, 1);
            x.op.next = 0b0000011.into();
            wait_clock_cycles!(ep, clock, x, 1);

            assert_eq!(x.alu_control.val(), 0b000);
            assert_eq!(x.imm_source.val(), 0b00);
            assert_eq!(x.mem_write.val(), false);
            assert_eq!(x.reg_write.val(), true);
            ep.done(x)
        });
        let uut = Control::default();

        sim.run_to_file(Box::new(uut), sim_time::ONE_SEC, "control.vcd")
            .unwrap();
        vcd_to_svg(
            "control.vcd",
            "control_all.svg",
            &["uut.clock"],
            0,
            sim_time::ONE_SEC,
        )
        .unwrap();
    }
}
