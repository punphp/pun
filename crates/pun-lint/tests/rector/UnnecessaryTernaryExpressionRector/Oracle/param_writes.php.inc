<?php

final class ParamWrites
{
    public function referencedThenWritten(bool $f)
    {
        $r = &$f;
        $r = 'x';
        return $f ? true : false;
    }

    public function foreachTarget(bool $f)
    {
        foreach ([1] as $f) {
        }
        return $f ? true : false;
    }

    public function readInLoop(bool $f, array $xs)
    {
        foreach ($xs as $x) {
            $y = $f ? true : false;
        }
        return $y;
    }

    public function globalled(bool $f)
    {
        global $f;
        return $f ? true : false;
    }
}
-----
<?php

final class ParamWrites
{
    public function referencedThenWritten(bool $f)
    {
        $r = &$f;
        $r = 'x';
        return (bool) $f;
    }

    public function foreachTarget(bool $f)
    {
        foreach ([1] as $f) {
        }
        return (bool) $f;
    }

    public function readInLoop(bool $f, array $xs)
    {
        foreach ($xs as $x) {
            $y = $f;
        }
        return $y;
    }

    public function globalled(bool $f)
    {
        global $f;
        return (bool) $f;
    }
}
