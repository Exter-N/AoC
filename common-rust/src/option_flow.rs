#[macro_export]
macro_rules! some_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_break {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return_none {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_continue {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => {
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_break {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => {
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => {
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_none {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => {
                return None;
            }
        }
    };
}
