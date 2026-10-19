<?php

// Rector prints `$this->a-b` and `$this->e'x`; Pun leaves the method alone.
class H
{
    public function __sleep()
    {
        return ['a', 'k' => 'v', 'a-b', 'e\'x'];
    }
}
