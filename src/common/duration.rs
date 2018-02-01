pub struct Duration {
    clock: u32,
    correction: u32,
}

impl Duration {

}

impl IntoPsc {
    fn into_psc(&self, clock: u32) -> (u16, u16) {
        tmp = clock / max(target, 65536);
        res = clock / tmp;
        return (tmp, (res - target));
    }
}

impl FromNs for Duration {

}

impl FromUs for Duration {

}

impl FromMs for Duration {

}

impl FromSec for Duration {

}