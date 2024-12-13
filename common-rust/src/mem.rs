use std::mem::take;

pub fn take2<T1: Default, T2: Default>(r1: &mut T1, r2: &mut T2) -> (T1, T2) {
    (take(r1), take(r2))
}

pub fn take_all2<T1, T2>(r1: &mut Option<T1>, r2: &mut Option<T2>) -> Option<(T1, T2)> {
    match (take(r1), take(r2)) {
        (Some(v1), Some(v2)) => Some((v1, v2)),
        (_, _) => None,
    }
}

pub fn take3<T1: Default, T2: Default, T3: Default>(
    r1: &mut T1,
    r2: &mut T2,
    r3: &mut T3,
) -> (T1, T2, T3) {
    (take(r1), take(r2), take(r3))
}

pub fn take_all3<T1, T2, T3>(
    r1: &mut Option<T1>,
    r2: &mut Option<T2>,
    r3: &mut Option<T3>,
) -> Option<(T1, T2, T3)> {
    match (take(r1), take(r2), take(r3)) {
        (Some(v1), Some(v2), Some(v3)) => Some((v1, v2, v3)),
        (_, _, _) => None,
    }
}

pub fn take4<T1: Default, T2: Default, T3: Default, T4: Default>(
    r1: &mut T1,
    r2: &mut T2,
    r3: &mut T3,
    r4: &mut T4,
) -> (T1, T2, T3, T4) {
    (take(r1), take(r2), take(r3), take(r4))
}

pub fn take_all4<T1, T2, T3, T4>(
    r1: &mut Option<T1>,
    r2: &mut Option<T2>,
    r3: &mut Option<T3>,
    r4: &mut Option<T4>,
) -> Option<(T1, T2, T3, T4)> {
    match (take(r1), take(r2), take(r3), take(r4)) {
        (Some(v1), Some(v2), Some(v3), Some(v4)) => Some((v1, v2, v3, v4)),
        (_, _, _, _) => None,
    }
}
