<?php

// Rector reads only the `if`'s own statements, so it drops the `else` of e(), and turns the by-reference loop
// variable of f() into a by-value arrow function parameter; Pun leaves both loops alone.
class A
{
    public function e(array $items)
    {
        foreach ($items as $item) {
            if ($item) {
                return false;
            } else {
                echo 1;
            }
        }
        return true;
    }

    public function f(array $items)
    {
        foreach ($items as &$item) {
            if ($item = 1) {
                return false;
            }
        }
        return TRUE;
    }
}
