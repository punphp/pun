<?php

class A
{
    public mixed $key {
        get {
            $found = null;
            foreach ([1, 'a'] as $key => $item) {
                if (is_int($item)) {
                    $found = $key;
                    break;
                }
            }
            return $found;
        }
    }
}
