<?php

final class TernaryShapes
{
    public function two($x)
    {
        if ($x) {
            return $x ? 1 : 2;
        }
        return $x ? 3 : 4;
    }

    public function bare($x)
    {
        return;
    }

    public function plain($x)
    {
        return 1;
    }
}
