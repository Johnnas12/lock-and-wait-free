// In lock-and-wait-free

use std::{
    ops::Index,
    slice::SliceIndex,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering},
};

pub struct ContentionMeasure(usize);

const CONTENTION_THRESHOLD: usize = 2;
const RETRY_THRESHOLD: usize = 2;

impl ContentionMeasure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }

    pub fn use_slow_path(&self) -> bool {
        self.0 > CONTENTION_THRESHOLD
    }
}
pub trait CasDescriptor {
    fn execute(&self) -> Result<(), ()>;
}

pub trait CasDescriptors<D>: Index<usize, Output = D>
where
    D: CasDescriptor,
{
    fn len(&self) -> usize;
}
pub trait NormalizedLockFree {
    type Input;
    type Output;
    type Cas: CasDescriptor;
    type Cases: CasDescriptors<Self::Cas>;
    fn generator(&self, op: &Self::Input, contention: &ContentionMeasure) -> Self::Cases;
    fn wrap_up(
        &self,
        executed: Result<(), usize>,
        performed: &Self::Cases,
        contention: &ContentionMeasure,
    ) -> Result<Self::Output, ()>;
}
pub struct OperationRecordBox<O> {
    val: AtomicPtr<OperationRecord>,
}
pub struct OperationRecord<O> {
    ownerTid: std::thread::ThreadId
    input: O
}

// A wait free queue
pub struct HelpQueue<O>;

impl<O> HelpQueue<O> {
    // Implement based on Appendix A
    pub fn enqueue(&self, help: *const OperationRecordBox<O>) {}

    pub fn peak(&self) -> Option<*const OperationRecordBox<O>> {
        None
    }

    pub fn try_remove_front(&self, completed: *const OperationRecordBox<O>) -> Result<(), ()> {
        Err(())
    }
}

pub struct WaitFreeSimulator<LF: NormalizedLockFree> {
    algorithim: LF,
    help: HelpQueue<LF::Input>,
}

impl<LF: NormalizedLockFree> WaitFreeSimulator<LF> {
    fn cas_execute(
        &self,
        descriptors: &LF::Cases,
        contention: &mut ContentionMeasure,
    ) -> Result<(), usize> {
        let len = descriptors.len();
        for i in 0..len {
            if descriptors[i].execute().is_err() {
                contention.detected();
                return Err(i);
            }
        }
        Ok(())
    }
    fn help_first(&self) {
        if let Some(help) = self.help.peak() {
            // help make progress
        }
    }
    pub fn run(&self, op: LF::Input) -> LF::Output {
        // Fast Path
        for retry in 0.. {
            let help = /* once in a while */ true;
            if help {
                self.help_first();
            }
            let mut contention = ContentionMeasure(0);

            let cases = self.algorithim.generator(&op, &mut contention);
            if contention.use_slow_path() {
                break;
            }
            let result = self.cas_execute(&cases, &mut contention);
            if contention.use_slow_path() {
                break;
            }
            match self.algorithim.wrap_up(result, &cases, &mut contention) {
                Ok(outcome) => break,
                Err(()) => continue,
            } // fast path

            if contention.use_slow_path() {
                break;
            }

            if retry > RETRY_THRESHOLD {
                // slow path
            }

            let i = 0;
            let orb = OperationRecordBox {
                val: AtomicPtr::new(Box::into_raw(Box::new(OperationRecord<O> {
                    completed: false,
                    at: i,
                }))),
            };

            self.help.enqueue(&orb);
            while !unsafe { &*orb.val.load(Ordering::SeqCst) }.completed {
                self.help_first();
            }
        }

        todo!()
    }
}
