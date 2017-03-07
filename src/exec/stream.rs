
#[macro_export]
macro_rules! chr_iter {
    ($c:expr, $t: expr) => {
        $c.store.iter().filter(|x| x.vtype == $t && !x.activated).collect::<Vec<_>>() 
    };
}

#[macro_export]
macro_rules! head_iter {
    ( $a:expr) => {
        $a
    };
    ( $a:expr, $b:expr) => {
        $a.cartesian_product($b)
    };
    ( $a:expr, $b:expr, $c:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
    };
    ( $a:expr, $b:expr, $c:expr, $d:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
        .cartesian_product($d)        
    };
    ( $a:expr, $b:expr, $c:expr, $d:expr, $e:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
        .cartesian_product($d)
        .cartesian_product($e)        
    };
    ( $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
        .cartesian_product($d)
        .cartesian_product($e)
        .cartesian_product($f)        
        .cartesian_product($g)
    };
    ( $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
        .cartesian_product($d)        
    };
    ( $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr ) => {
        $a.cartesian_product($b)
        .cartesian_product($c)
        .cartesian_product($d)    
        .cartesian_product($e)    
        .cartesian_product($f)    
        .cartesian_product($g)    
        .cartesian_product($h)        
        .cartesian_product($i)    
    };
}