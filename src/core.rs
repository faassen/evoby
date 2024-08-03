// the executor trait can execute actual instructions
trait Executor {
    fn call(&mut self);
    fn return_(&mut self);
    fn value(&mut self); // this instruction actually would need to take a register too
    fn if_(&mut self, r0: u8);
    fn repeat(&mut self, r0: u8);
    fn not(&mut self, r0: u8);
    fn push(&mut self, r0: u8);
    fn pop(&mut self, r0: u8);
    fn inc(&mut self, r0: u8);
    fn dec(&mut self, r0: u8);
    fn store(&mut self, r0: u8, r1: u8);
    fn load(&mut self, r0: u8, r1: u8);
    fn add(&mut self, r0: u8, r1: u8);
    fn sub(&mut self, r0: u8, r1: u8);
    fn mul(&mut self, r0: u8, r1: u8);
    fn div(&mut self, r0: u8, r1: u8);
    fn eq(&mut self, r0: u8, r1: u8);
    fn gt(&mut self, r0: u8, r1: u8);
    fn and(&mut self, r0: u8, r1: u8);
    fn or(&mut self, r0: u8, r1: u8);
    fn xor(&mut self, r0: u8, r1: u8);
    fn unknown0(&mut self, r0: u8, r1: u8);
    fn unknown1(&mut self, r0: u8, r1: u8);
}

// decode and execute a byte instruction
// we do this with simple direct decoding
fn execute<E: Executor>(executor: &mut E, instruction: u8) {
    // first decode the opcode
    let opcode = instruction >> 4;
    match opcode {
        0 => execute_special0(executor, instruction),
        1 => execute_special1(executor, instruction),
        opcode @ 2..=14 => {
            // the operand is the low nibble, consisting of two two-bit registers
            let r0 = instruction & 0b0000_1100 >> 2;
            let r1 = instruction & 0b0000_0011;
            match opcode {
                2 => executor.store(r0, r1),
                3 => executor.load(r0, r1),
                4 => executor.add(r0, r1),
                5 => executor.sub(r0, r1),
                6 => executor.mul(r0, r1),
                7 => executor.div(r0, r1),
                8 => executor.eq(r0, r1),
                9 => executor.gt(r0, r1),
                10 => executor.and(r0, r1),
                11 => executor.or(r0, r1),
                12 => executor.xor(r0, r1),
                13 => executor.unknown0(r0, r1),
                14 => executor.unknown1(r0, r1),
                _ => unreachable!(),
            }
        }
        15 => {
            // pattern is a no-op
        }
        _ => unreachable!(),
    }
}

fn execute_special0<E: Executor>(executor: &mut E, instruction: u8) {
    match instruction {
        0 => {
            // block is a no-op
        }
        1 => {
            executor.call();
        }
        2 => {
            executor.return_();
        }
        3 => {
            executor.value();
        }
        4..=7 => {
            executor.if_(register_operand(instruction));
        }
        8..=11 => {
            executor.repeat(register_operand(instruction));
        }
        12..=15 => {
            executor.not(register_operand(instruction));
        }
        _ => unreachable!(),
    }
}

fn execute_special1<E: Executor>(executor: &mut E, instruction: u8) {
    match instruction {
        0..=3 => {
            executor.push(register_operand(instruction));
        }
        4..=7 => {
            executor.pop(register_operand(instruction));
        }
        8..=11 => {
            executor.inc(register_operand(instruction));
        }
        12..=15 => {
            executor.dec(register_operand(instruction));
        }
        _ => unreachable!(),
    }
}

fn register_operand(instruction: u8) -> u8 {
    instruction & 0b0000_0011
}

struct Processor {}

impl Processor {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestExecutor {
        trace: Vec<String>,
    }

    impl TestExecutor {
        fn new() -> TestExecutor {
            TestExecutor { trace: Vec::new() }
        }
    }

    impl Executor for TestExecutor {
        fn call(&mut self) {
            self.trace.push("call".to_string());
        }
        fn return_(&mut self) {
            self.trace.push("return".to_string());
        }
        fn value(&mut self) {
            self.trace.push("value".to_string());
        }
        fn if_(&mut self, r0: u8) {
            self.trace.push(format!("if {}", r0));
        }
        fn repeat(&mut self, r0: u8) {
            self.trace.push(format!("repeat {}", r0));
        }
        fn not(&mut self, r0: u8) {
            self.trace.push(format!("not {}", r0));
        }
        fn push(&mut self, r0: u8) {
            self.trace.push(format!("push {}", r0));
        }
        fn pop(&mut self, r0: u8) {
            self.trace.push(format!("pop {}", r0));
        }
        fn inc(&mut self, r0: u8) {
            self.trace.push(format!("inc {}", r0));
        }
        fn dec(&mut self, r0: u8) {
            self.trace.push(format!("dec {}", r0));
        }
        fn store(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("store {} {}", r0, r1));
        }
        fn load(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("load {} {}", r0, r1));
        }
        fn add(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("add {} {}", r0, r1));
        }
        fn sub(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("sub {} {}", r0, r1));
        }
        fn mul(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("mul {} {}", r0, r1));
        }
        fn div(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("div {} {}", r0, r1));
        }
        fn eq(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("eq {} {}", r0, r1));
        }
        fn gt(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("gt {} {}", r0, r1));
        }
        fn and(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("and {} {}", r0, r1));
        }
        fn or(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("or {} {}", r0, r1));
        }
        fn xor(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("xor {} {}", r0, r1));
        }
        fn unknown0(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("unknown0 {} {}", r0, r1));
        }
        fn unknown1(&mut self, r0: u8, r1: u8) {
            self.trace.push(format!("unknown1 {} {}", r0, r1));
        }
    }

    #[test]
    fn test_execute_call() {
        let mut executor = TestExecutor::new();
        execute(&mut executor, 0b0000_0001);
        assert_eq!(executor.trace, vec!["call".to_string()]);
    }

    #[test]
    fn test_execute_if_r0() {
        let mut executor = TestExecutor::new();
        execute(&mut executor, 0b0000_0100);
        assert_eq!(executor.trace, vec!["if 0".to_string()]);
    }

    #[test]
    fn test_execute_if_r1() {
        let mut executor = TestExecutor::new();
        execute(&mut executor, 0b0000_0101);
        assert_eq!(executor.trace, vec!["if 1".to_string()]);
    }

    #[test]
    fn test_execute_if_r2() {
        let mut executor = TestExecutor::new();
        execute(&mut executor, 0b0000_0110);
        assert_eq!(executor.trace, vec!["if 2".to_string()]);
    }

    #[test]
    fn test_execute_if_r3() {
        let mut executor = TestExecutor::new();
        execute(&mut executor, 0b0000_0111);
        assert_eq!(executor.trace, vec!["if 3".to_string()]);
    }
}
