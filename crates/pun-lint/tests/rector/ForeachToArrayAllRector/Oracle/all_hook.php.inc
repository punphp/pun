<?php

class A
{
    public bool $all {
        get {
            $found = true;
            foreach ([1, 'a'] as $item) {
                if (!is_int($item)) {
                    $found = false;
                    break;
                }
            }
            return $found;
        }
    }
}
