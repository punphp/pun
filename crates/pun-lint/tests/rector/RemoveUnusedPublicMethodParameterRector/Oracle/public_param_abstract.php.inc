<?php

final class AbstractInFinal
{
    abstract public function f($a);
}
-----
<?php

final class AbstractInFinal
{
    abstract public function f();
}
