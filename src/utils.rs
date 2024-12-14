#[macro_export]
#[doc(hidden)]
macro_rules! max {
    ($x:block) => ( $x );
    ($x:block, $($xs:block),+) => {
        {
            let a = $x;
            let b = $crate::max!( $($xs),+ );
            if a > b {
                a
            }else {
                b
            }
        }
    };
}
