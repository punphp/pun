<?php

namespace Pun\ReturnBool;

final class OperandParentheses
{
    public function ternaryLeaf($a, $b)
    {
        if (($a ?: $b) && $b) {
            return false;
        }
        return true;
    }

    public function ternaryCast($a, $b)
    {
        if ($a ?: $b) {
            return true;
        }
        return false;
    }
}
