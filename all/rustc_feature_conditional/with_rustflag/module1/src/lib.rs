
// #[ cfg( feature = "nightly" ) ]
#[ cfg( nightly ) ]
#[ macro_export ]
macro_rules! name_of
{
  ( $src : expr ) =>
  {{
    std::any::type_name_of_val( &$src )
  }};
}
