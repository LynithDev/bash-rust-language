#[macro_export]
macro_rules! place_first_arg_ignore_rest_if_any {
    ($mandatory:tt $(, $optional:tt)* $(,)?) => {
        $mandatory
    };
}

#[macro_export]
macro_rules! enum_str {
    ($(#[$attr:meta])*
     $vis:vis enum $name:ident $(<$($gen:ident),*>)?,
     $(
         $variant:ident
             $( (
                 $($tfield:ty),*
                 $(,)?
             ) )?
             $( {
                 $($sfield:ident: $stype:ty),*
                 $(,)?
             } )?
     ),*
     $(,)?
     ) => {
        $(#[$attr])*
        $vis enum $name $(<$($gen),*>)? {
            $(
                $variant $( ( $($tfield),* ) )?
                         $( { $($sfield: $stype),* })?
            ),*
        }//enum

        //impl $(<$($gen),*>)? VariantNameAsStr for $name $(<$($gen),*>)? {
        impl $(<$($gen),*>)? $name $(<$($gen),*>)? {
            pub const fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        // Handle variants with fields
                        Self::$variant $( ( $crate::replace_with_2_dots!( $($tfield),* ) ) )?
                                       $( { $($sfield: _),* } )?
                        => stringify!($variant),
                    )*
                }//match
            }//fn
        }//impl
    };//arm
} //macro