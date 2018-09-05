use std::sync::atomic::{AtomicUsize, Ordering};

use ctxt::SemContext;
use driver::cmd::Args;
use gc::Address;
use gc::{arena, Collector};

pub struct ZeroCollector {
    start: Address,
    end: Address,
    next: AtomicUsize,
}

impl ZeroCollector {
    pub fn new(args: &Args) -> ZeroCollector {
        let heap_size: usize = args.heap_size();

        let ptr = arena::reserve(heap_size);
        arena::commit(ptr, heap_size, false);

        ZeroCollector {
            start: ptr,
            end: ptr.offset(heap_size),
            next: AtomicUsize::new(ptr.to_usize()),
        }
    }
}

impl Collector for ZeroCollector {
    fn alloc(&self, _ctxt: &SemContext, size: usize, _array_ref: bool) -> Address {
        let mut old = self.next.load(Ordering::Relaxed);
        let mut new;

        loop {
            new = old + size;

            if new >= self.end.to_usize() {
                return Address::null();
            }

            let res =
                self.next
                    .compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed);

            match res {
                Ok(_) => break,
                Err(x) => old = x,
            }
        }

        old.into()
    }

    fn collect(&self, _: &SemContext) {
        // do nothing
    }

    fn minor_collect(&self, _: &SemContext) {
        // do nothing
    }
}
