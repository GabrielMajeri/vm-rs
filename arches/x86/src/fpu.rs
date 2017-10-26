/// Contains the x87 FPU and SSE state.
#[derive(Debug, Default, Copy, Clone)]
pub struct State {
    /// The control word.
    pub control: ControlWord,
    /// The status word.
    pub status: StatusWord,
}

bitflags! {
    /// The control word for the FPU.
    ///
    /// It is used to mask exceptions, control operation precision,
    /// rounding mode or infinity.
    #[derive(Default)]
    pub struct ControlWord: u16 {
        /// Exception mask for invalid operation errors.
        const INVALID_OPERATION = 1 << 0;
        /// Exception mask for denormalized operand errors.
        const DENORMALIZED_OPERAND = 1 << 1;
        /// Exception mask for divide-by-zero errors.
        const ZERO_DIVIDE = 1 << 2;
        /// Exception mask for overflow errors.
        const OVERFLOW = 1 << 3;
        /// Exception mask for underflow errors.
        const UNDERFLOW = 1 << 4;
        /// Exception mask for precision errors.
        const PRECISION = 1 << 5;

        /// 32-bit (float).
        const SINGLE_PRECISION = 0 << 8;
        /// 64-bit (double).
        const DOUBLE_PRECISION = 1 << 9;
        /// 80-bit (long double).
        const EXTENDED_PRECISION = 0b11 << 8;

        /// Round to nearest even.
        const ROUND = 0 << 10;
        /// Round down towards infinity.
        const ROUND_DOWN = 1 << 10;
        /// Round up towards infinity.
        const ROUND_UP = 1 << 11;
        /// Round to zero / truncate.
        const ROUND_TRUNCATE = 0b11 << 10;

        /// Projective infinity.
        const PROJECTIVE_INF = 0 << 12;
        /// Affine infinity.
        const AFFINE_INF = 1 << 12;
    }
}

bitflags! {
    /// The status word of the FPU.
    #[derive(Default)]
    pub struct StatusWord: u16 {
        /// An operation was invalid.
        const INVALID_OPERATION = 1 << 0;
        /// Attempt to operate on a denormalized number.
        const DENORMALIZED_OPERAND = 1 << 1;
        /// Attempt to divide by 0.
        const ZERO_DIVIDE = 1 << 2;
        /// Value was too large in magnitude to be represented properly.
        const OVERFLOW = 1 << 3;
        /// Value was too small.
        const UNDERFLOW = 1 << 4;
        /// Some precision would be lost when executing the operation.
        const PRECISION = 1 << 5;
        /// Attempt to load a value in an in-use register, or pop a free register.
        const STACK_FAULT = 1 << 6;

        /// Set while an exception is being handled.
        const INTERRUPT_REQUEST = 1 << 7;

        /// Condition code 0.
        const C0 = 1 << 8;
        /// Condition code 1.
        const C1 = 1 << 9;
        /// Condition code 2.
        const C2 = 1 << 10;

        /// The index of the stack's top register.
        const TOP = 0b111 << 11;

        /// Condition code 3.
        const C3 = 1 << 14;

        /// Set if FPU is busy.
        const BUSY = 1 << 15;
    }
}
