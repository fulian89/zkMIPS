use p3_field::{Field, FieldAlgebra};
use zkm_core_executor::{events::ByteRecord, ByteOpcode, ExecutionRecord};
use zkm_derive::AlignedBorrow;
use zkm_primitives::consts::WORD_SIZE;
use zkm_stark::{air::ZKMAirBuilder, Word};

/// A set of columns needed to compute the or of two words.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct OrOperation<T> {
    /// The result of `x | y`.
    pub value: Word<T>,
}

impl<F: Field> OrOperation<F> {
    pub fn populate(&mut self, record: &mut ExecutionRecord, x: u32, y: u32) -> u32 {
        let expected = x | y;
        let x_bytes = x.to_le_bytes();
        let y_bytes = y.to_le_bytes();
        for i in 0..WORD_SIZE {
            self.value[i] = F::from_canonical_u8(x_bytes[i] | y_bytes[i]);
            record.lookup_or(x_bytes[i], y_bytes[i]);
        }
        expected
    }

    pub fn eval<AB: ZKMAirBuilder>(
        builder: &mut AB,
        a: Word<AB::Var>,
        b: Word<AB::Var>,
        cols: OrOperation<AB::Var>,
        is_real: AB::Var,
    ) {
        for i in 0..WORD_SIZE {
            builder.send_byte(
                AB::F::from_canonical_u32(ByteOpcode::OR as u32),
                cols.value[i],
                a[i],
                b[i],
                is_real,
            );
        }
    }
}
