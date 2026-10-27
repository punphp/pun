<?php

namespace N;

ldap_first_attribute($a, $b, $c);
\LDAP_FIRST_ATTRIBUTE($a, $b, $c, $d);
ldap_first_attribute($a, $b);
ldap_first_attribute(
    $a,
    $b,
    $c
);
ldap_first_attribute($a, $b, entry: $c);
ldap_first_attribute($a, ...$b);
-----
<?php

namespace N;

ldap_first_attribute($a, $b);
\LDAP_FIRST_ATTRIBUTE($a, $b, $d);
ldap_first_attribute($a, $b);
ldap_first_attribute(
    $a,
    $b
);
ldap_first_attribute($a, $b);
ldap_first_attribute($a, ...$b);
