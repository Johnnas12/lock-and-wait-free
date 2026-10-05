// In lock-and-wait-free

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


pub struct WaitFreeSimulator <LF: NormalizedLockFree> {
    algorithim: LF,

}

impl <LF: NormalizedLockFree>  WaitFreeSimulator<LF>  {
    pub fn run(&self, op: LF::Operation) -> LF::Output {
        let mut contention = ContentionMeasure(0);
        let cas = self.algorithim.prepare(&op);
        match self.algorithim.execute(&cas[..], &mut contention) {
            Ok(()) => {
                self.algorithim.cleanup(); // fast path is succeed
            }
            Err(i) => { 
                // slow-path: ask for help
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