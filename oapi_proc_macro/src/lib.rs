use synstructure;

mod check;

use check::oapi_check_derive;

synstructure::decl_derive!([OApiCheck] => oapi_check_derive);
