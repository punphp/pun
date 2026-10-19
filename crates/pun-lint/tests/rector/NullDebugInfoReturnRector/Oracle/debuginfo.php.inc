<?php

class A
{
    public function __DEBUGINFO()
    {
        if ($this->x) {
            return NULL;
        }
        $f = function () {
            return null;
        };
        $o = new class {
            public function x()
            {
                return null;
            }
        };
        $g = fn () => null;
        return (null);
    }
}

interface B
{
    public function __debugInfo();
}

abstract class C
{
    abstract public function __debugInfo();

    public function other()
    {
        return null;
    }
}
-----
<?php

class A
{
    public function __DEBUGINFO()
    {
        if ($this->x) {
            return [];
        }
        $f = function () {
            return null;
        };
        $o = new class {
            public function x()
            {
                return null;
            }
        };
        $g = fn () => null;
        return ([]);
    }
}

interface B
{
    public function __debugInfo();
}

abstract class C
{
    abstract public function __debugInfo();

    public function other()
    {
        return null;
    }
}
