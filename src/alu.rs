use rust_hdl::prelude::*;

const REGS: usize = 32;

#[derive(LogicBlock)]
pub struct Alu {
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
