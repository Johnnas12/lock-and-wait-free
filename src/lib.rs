// In lock-and-wait-free

use std::{ops::Index, slice::SliceIndex, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};

pub struct ContentionMeasure (usize);

impl ContentionMeasure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }
}
pub struct CasDescriptor;

pub trait CasDescriptors: Index<usize, Output = CasDescriptor> {
    fn len(&self) -> usize;
}
pub trait NormalizedLockFree {
    type Input;
    type Output;
    type Descriptor: CasDescriptors;

    fn generator(&self, op: &Self::Input) -> Self::Descriptor;
    fn wrap_up(&self, performed: Self::Descriptor);
}
pub struct Help{
    completed: AtomicBool,
    at: AtomicUsize,

}

// A wait free queue
pub struct HelpQueue;

impl HelpQueue {
    pub fn add(&self, help: *const Help) {

    }

    pub fn peak(&self) -> Option<*const Help> {
        None
    }

    pub fn try_remove_front(&self, completed: *const Help) {

    }

}


pub struct WaitFreeSimulator <LF: NormalizedLockFree> {
    algorithim: LF,
    help: HelpQueue

}

impl <LF: NormalizedLockFree>  WaitFreeSimulator<LF>  {
    fn cas_executor(&self, descriptors: LF::Descriptor) -> Result<(), usize> {
        todo!()
    }
    fn help(&self) {
        if let Some(help) = self.help.peak() {
            // help make progress
        }

    }
    pub fn run(&self, op: LF::Input) -> LF::Output {
        if false {
            self.help();
        }
        let mut contention = ContentionMeasure(0);
        let cas = self.algorithim.prepare(&op);
        match self.algorithim.execute(&cas[..], &mut contention) {
            Ok(()) => {
                self.algorithim.cleanup(); // fast path is succeed
            }
            Err(i) => { 
                // slow-path: ask for help
                let help = Help{
                    completed: AtomicBool::new(false),
                    at: AtomicUsize::new(i)
                } ;
                self.help.add(&help);

                while !help.completed.load(Ordering::SeqCst) {
                    self.help();
                }
            }
        }
        

        todo!()
    }
    
}