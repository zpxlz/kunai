use crate::bpf_target_code;

pub const KUNAI_EVENTS_MAP: &str = "KUNAI_EVENTS";
pub const KUNAI_STATS_MAP: &str = "KUNAI_STATS";

bpf_target_code! {
    use super::{Event,Type};
    use aya_bpf::{macros::map, maps::{HashMap,PerfEventByteArray}, BpfContext};

    #[map(name = "KUNAI_EVENTS")]
    static mut EVENTS: PerfEventByteArray = PerfEventByteArray::with_max_entries(0x1ffff, 0);

    #[map(name = "KUNAI_STATS")]
    static mut STATS: HashMap<Type, usize> = HashMap::with_max_entries(Type::Max as u32, 0);

    pub unsafe fn pipe_event<C: BpfContext, T>(ctx: &C, e: &Event<T>) {
        match STATS.get_ptr_mut(&e.ty()){
            Some(e) => {*e += 1},
            None => {
                // we ignore results
                let _ = STATS.insert(&e.ty(), &1, 0);
                },
        }
        EVENTS.output(ctx, e.encode(), 0);
    }
}
