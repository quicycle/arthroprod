// Easier variadic generation of alpha values including setting sign
#[macro_export]
macro_rules! alpha(
    ($($num:expr) +) => {
        {
            let mut sign = Sign::Pos;
            let mut axes = Vec::new();
            $(axes.push(Axis::try_from_u8($num).unwrap());)+

            Alpha::try_from_axes(sign, &axes).unwrap()
        }
    };
);

#[macro_export]
macro_rules! term(
    ($($num:expr) +) => {
        {
            let mut sign = Sign::Pos;
            let mut axes = Vec::new();
            $(axes.push(Axis::try_from_u8($num).unwrap());)+
            let alpha = Alpha::try_from_axes(sign, &axes).unwrap();

            Term::from_alpha(alpha)
        }
    };

    ($sym:tt, $($num:expr) +) => {
        {
            let mut sign = Sign::Pos;
            let mut axes = Vec::new();
            let symbol = String::from($sym);
            $(axes.push(Axis::try_from_u8($num).unwrap());)+
            let alpha = Alpha::try_from_axes(sign, &axes).unwrap();

            Term::new(symbol, alpha)
        }
    };
);

#[macro_export]
macro_rules! mvec(
    [$($ar_elem:expr),+] => {
        {
            let mut terms = Vec::new();
            $(terms.extend($ar_elem.as_terms());)+

            MultiVector::from_terms(terms)
        }
    };
);

// Helper for making a map literal
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(_map.insert($key, $value);)+
            _map
        }
    };
);
