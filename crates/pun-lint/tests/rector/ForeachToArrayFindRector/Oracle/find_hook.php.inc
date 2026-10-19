<?php

class A
{
    public mixed $first {
        get {
            $found = null;
            foreach ([1, 'a'] as $item) {
                if (is_int($item)) {
                    $found = $item;
                    break;
                }
            }
            return $found;
        }
    }
}
