use rust_hdl::prelude::*;

#[derive(LogicBlock)]
pub struct SignExt {
    pub clock: Signal<In, Clock>,

    // raw instruction 
    pub raw: Signal<In, Bits<25>>,
    // position of imm
    pub imm_src: Signal<In, Bits<2>>,
    // sign extended imm
    pub immediate: Signal<Out, Bits<32>>,

    // temporary field for imm
    gathered_imm: Signal<Local, Bits<12>>,
}

impl Default for SignExt {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            raw: Default::default(),
            imm_src: Default::default(),
            immediate: Default::default(),
            gathered_imm: Default::default(),
        }
    }
}

impl Logic for SignExt {
    #[hdl_gen]
    fn update(&mut self) {
        if self.imm_src.val() == 0b00 {
            self.gathered_imm.next = self.raw.val().get_bits::<12>(13);
        } else {
            self.gathered_imm.next = 0.into();
        }

        if self.gathered_imm.val().get_bit(11) {
            self.immediate.next =
                bit_cast::<32, 12>(self.gathered_imm.val()) | Bits::<32>::from(0xFFFFF000u64);
        } else {
            self.immediate.next = bit_cast::<32, 12>(self.gathered_imm.val());
        }
    }
}

#[cfg(test)]
mod signext_test {
    use rust_hdl::prelude::*;

    use crate::CLOCK_SPEED_HZ;

    use super::SignExt;

    #[test]
    fn signext_test() {
        let mut sim = simple_sim!(SignExt, clock, CLOCK_SPEED_HZ, ep, {
            let mut x = ep.init()?;

            // imm test
            let mut imm = 100;
            imm <<= 13;

            let rand_junk = 0b000000000000_1111111111111;

            wait_clock_cycles!(ep, clock, x, 1);
            x.raw.next = (imm | rand_junk).into();
            x.imm_src.next = 0.into();
            wait_clock_cycles!(ep, clock, x, 1);

            assert_eq!(x.immediate.val(), 100);

            // neg imm
            let mut imm = 0b111110011100; // -100
            imm <<= 13;

            let rand_junk = 0b000000000000_1111111111111;

            wait_clock_cycles!(ep, clock, x, 1);
            x.raw.next = (imm | rand_junk).into();
            x.imm_src.next = 0.into();
            wait_clock_cycles!(ep, clock, x, 1);

            assert_eq!(x.immediate.val(), 0b11111111111111111111111110011100);
            ep.done(x)
        });
        let uut = SignExt::default();

        sim.run_to_file(Box::new(uut), sim_time::ONE_SEC, "signext.vcd")
            .unwrap();
        vcd_to_svg(
            "signext.vcd",
            "signext_all.svg",
            &["uut.clock", "uut.immediate"],
            0,
            sim_time::ONE_SEC,
        )
        .unwrap();
    }
}
