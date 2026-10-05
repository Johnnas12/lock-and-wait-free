// In lock-and-wait-free

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct ContentionMeasure (usize);

impl ContentionMeasure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }
}
pub trait NormalizedLockFree {
    type Output;
    type Operation;
    type CasDescriptor;

    fn prepare(&self, op: &Self::Operation) -> [Self::CasDescriptor; 1];
    fn execute(&self, cases: &[Self::CasDescriptor], contention: &mut ContentionMeasure) -> Result<(), usize>;
    fn cleanup(&self);
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
    fn help(&self) {
        if let Some(help) = self.help.peak() {
            // help make progress
        }

    }
    pub fn run(&self, op: LF::Operation) -> LF::Output {
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

// // in consuming create(wait-free-linked-list crate)
// struct WaitFreeLinkedList<T> {
//     simulator: WaitFreeSimulator<LockFreeLinkedList<T>>
// }

// struct LockFreeLinkedList<T> {
//     t: T
// }

// impl<T> NormalizedLockFree for LockFreeLinkedList<T> {}

// impl<T> WaitFreeLinkedList<T> {
//     pub fn push_front(&self, t:T) {
//         self.simulator.run(Insert(t));
//     }
// }