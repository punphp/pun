<?php

namespace N;

use function A\pi;

function php_sapi_name() {}

class C
{
    public function run()
    {
        $a = \pi();
        $b = pi();
        $c = PHP_SAPI_NAME();
        $d = php_sapi_name();
        $e = \php_sapi_name() . 'x';
    }
}
-----
<?php

namespace N;

use function A\pi;

function php_sapi_name() {}

class C
{
    public function run()
    {
        $a = M_PI;
        $b = pi();
        $c = PHP_SAPI_NAME();
        $d = php_sapi_name();
        $e = PHP_SAPI . 'x';
    }
}
