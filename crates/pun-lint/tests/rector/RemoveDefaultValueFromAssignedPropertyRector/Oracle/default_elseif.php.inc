<?php

final class BranchElseIf
{
    private int $a = 0;

    public function __construct(int $x)
    {
        if ($x > 1) {
            $this->a = 1;
        } elseif ($x > 0) {
            $this->a = 3;
        } else {
            $this->a = 2;
        }
    }
}
