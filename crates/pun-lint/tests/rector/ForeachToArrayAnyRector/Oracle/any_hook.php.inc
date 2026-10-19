<?php

class A
{
    public bool $any {
        get {
            $found = false;
            foreach ([1, 'a'] as $item) {
                if (is_int($item)) {
                    $found = true;
                    break;
                }
            }
            return $found;
        }
    }
}
