// A bounded handoff on the game thread between network polling and the
// corresponding pickup animation. Nothing in the packet stream is changed.
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Receipt { session: usize, item: i32, player: i32, amount: u32 }
thread_local! { static RECEIPTS: RefCell<VecDeque<Receipt>> = RefCell::new(VecDeque::new()); }

pub fn observe(session: &crate::net::Session, events: &[crate::net::Event]) {
    use horizon_protocol::packets::play::Clientbound;
    RECEIPTS.with(|receipts| {
        let mut receipts = receipts.borrow_mut();
        for event in events {
            if let crate::net::Event::Packet(packet) = event {
                if let Clientbound::TakeItemEntity(p) = &**packet {
                    if p.amount > 0 {
                        if receipts.len() == 256 { receipts.pop_front(); }
                        receipts.push_back(Receipt { session: session as *const _ as usize,
                            item: p.item, player: p.player, amount: p.amount as u32 });
                    }
                }
            }
        }
    });
}

pub fn take(session: &crate::net::Session, item: u32, player: i32) -> Option<u32> {
    RECEIPTS.with(|receipts| {
        let mut receipts = receipts.borrow_mut();
        let key = session as *const _ as usize;
        let index = receipts.iter().position(|r| r.session == key && r.item == item as i32 && r.player == player)?;
        receipts.remove(index).map(|receipt| receipt.amount)
    })
}

pub fn clear() { RECEIPTS.with(|receipts| receipts.borrow_mut().clear()); }
