<?php

namespace App;

final class NativeSkips
{
    public function noReturn(string $s)
    {
        echo strlen($s);
    }

    public function bareReturn(string $s)
    {
        if ($s === '') {
            return;
        }

        return strlen($s);
    }

    public function notCall(string $s)
    {
        if ($s === '') {
            return 0;
        }

        return strlen($s);
    }

    public function mixedCall(string $s)
    {
        return json_decode($s);
    }

    public function mixedUnion(string $s)
    {
        if ($s === '') {
            return strlen($s);
        }

        return json_decode($s);
    }
}
