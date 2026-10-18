<?php

class Hook
{
    public int $a {
        get {
            if ($this->b) {
                $x = 1;
            }
            return $x;
        }
    }

    public function property($a)
    {
        if ($a) {
            $this->x = 1;
        }
        return $this->x;
    }

    public function propertyAssign($a)
    {
        if ($a) {
            $this->x = 1;
        }
        return $x;
    }
}

function noValue($a)
{
    if ($a) {
        $x = 1;
    }
    return;
}

function notIfBefore($a)
{
    foo();
    return $a;
    if ($a) {
        $a = 1;
    }
    return $a;
}

function withElse($a)
{
    if ($a) {
        $x = 1;
    } else {
        $x = 2;
    }
    return $x;
}

function variableVariable($a, $name)
{
    if ($a) {
        $$name = 1;
    }
    return $name;
}

function otherVariable($a)
{
    if ($a) {
        $y = 1;
    }
    return $x;
}
-----
<?php

class Hook
{
    public int $a {
        get {
            if ($this->b) {
                $x = 1;
            }
            return $x;
        }
    }

    public function property($a)
    {
        if ($a) {
            $this->x = 1;
        }
        return $this->x;
    }

    public function propertyAssign($a)
    {
        if ($a) {
            $this->x = 1;
        }
        return $x;
    }
}

function noValue($a)
{
    if ($a) {
        $x = 1;
    }
    return;
}

function notIfBefore($a)
{
    foo();
    return $a;
    if ($a) {
        return 1;
    }
    return $a;
}

function withElse($a)
{
    if ($a) {
        $x = 1;
    } else {
        $x = 2;
    }
    return $x;
}

function variableVariable($a, $name)
{
    if ($a) {
        $$name = 1;
    }
    return $name;
}

function otherVariable($a)
{
    if ($a) {
        $y = 1;
    }
    return $x;
}
