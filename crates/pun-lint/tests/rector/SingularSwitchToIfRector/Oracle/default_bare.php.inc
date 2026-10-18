<?php

if ($a) switch ($b) { default: a(); b(); }
-----
<?php

if ($a) {
    a();
    b();
}
