// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state
struct Luggage<State> {
    id: u64,
    state: State,
}

impl<State> Luggage<State> {
    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            id: self.id,
            state,
        }
    }
}

struct CheckIn;
impl Luggage<CheckIn> {
    fn new(id: u64) -> Luggage<CheckIn> { Luggage { id, state: CheckIn, }}
    fn checkin(self) -> Luggage<OnLoading> { self.transition(OnLoading) }
}
struct OnLoading;
impl Luggage<OnLoading> {
    fn onload(self) -> Luggage<Offloading> { self.transition(Offloading) }
}
struct Offloading;
impl Luggage<Offloading> {
    fn offload(self) -> Luggage<AwaitingPickup> { self.transition(AwaitingPickup) }
}
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn pickup(self) -> Luggage<EndCustody> { self.transition(EndCustody) }
}
struct EndCustody;
impl Luggage<EndCustody> {
    fn confirm(self) { println!("Customer confirm own luggage now!"); }
}
// solution 
#[derive(Copy, Clone)]
struct LuggageId(usize);

struct LuggageNew(LuggageId);

impl LuggageNew {
    fn new(id: LuggageId) -> Self {
        LuggageNew(id)
    }
    fn check_in(self) -> CheckInNew {
        CheckInNew(self.0)
    }
}
struct CheckInNew(LuggageId);
impl CheckInNew {
    fn onload(self) -> OnLoadNew {
        OnLoadNew(self.0)
    }
}
struct OnLoadNew(LuggageId);
impl OnLoadNew {
    fn offload(self) -> OffLoadNew {
        OffLoadNew(self.0)
    }
}
struct OffLoadNew(LuggageId);
impl OffLoadNew {
    fn carousel(self) -> AwaitingPickupNew {
        AwaitingPickupNew(self.0)
        }
}
struct AwaitingPickupNew(LuggageId);
impl AwaitingPickupNew {
    fn pickup(self) -> (LuggageNew, EndCustodyNew) {
        (LuggageNew(self.0), EndCustodyNew(self.0))
    }
}
struct EndCustodyNew(LuggageId);


fn main() {
    //let lug = Luggage::<CheckIn>::new(1);
    let lug = Luggage::new(1);
    lug.checkin().onload().offload().pickup().confirm();
    // solution
    let id = LuggageId(1);
    let luggage = LuggageNew::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    let (luggage, _) = luggage.pickup();
}
