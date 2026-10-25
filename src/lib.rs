// In lock-and-wait-free

use std::{ops::Index, slice::SliceIndex, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};

pub struct ContentionMeasure (usize);

impl ContentionMeasure {
    pub fn detected(&mut self) {
        self.0 += 1;
    }
}
pub trait CasDescriptor {
    fn execute(&self) -> Result<(), ()>;
}

pub trait CasDescriptors<D>: Index::<usize, Output = D> where
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
    fn wrap_up(&self, 
        executed: Result<(), usize>, 
        performed: &Self::Cases,
        contention: &ContentionMeasure) -> Result<Self::Output, ()>;
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
    fn cas_execute(&self, descriptors: &LF::Cases, contention: &mut ContentionMeasure) -> Result<(), usize> {
        let len = descriptors.len();
        for i in 0..len {
            if descriptors[i].execute().is_err() {
                contention.detected();
                return Err(i);
            }
        }
        Ok(())
        
        }
    fn help(&self) {
        if let Some(help) = self.help.peak() {
            // help make progress
        }

    }
    pub fn run(&self, op: LF::Input) -> LF::Output {

        let mut fast = true;
        loop {
            if fast {
                let help = /* once in a while */ false;
                if help {
                    self.help();
                } 
            } else {
                // help more
            }
            fast = false;

            let mut contention = ContentionMeasure(0);
            let cases = self.algorithim.generator(&op, &mut contention);
            let result =  self.cas_execute(&cases, &mut contention);
            match self.algorithim.wrap_up(result, &cases, &mut contention){
                Ok(outcome ) => break,
                Err(()) => continue,
            } // fast path is succeed
            // if let Err(i) = result { 
            //         // slow-path: ask for help
            //         let help = Help{
            //             completed: AtomicBool::new(false),
            //             at: AtomicUsize::new(i)
            //         } ;
            //         self.help.add(&help);

            //         while !help.completed.load(Ordering::SeqCst) {
            //             self.help();
            //         }
            //     }
        }
        

        todo!()
    }
    
}