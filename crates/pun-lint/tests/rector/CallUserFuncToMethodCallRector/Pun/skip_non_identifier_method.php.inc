<?php

// Rector prints `$this->a-b()`, a subtraction; Pun leaves the call.
class A
{
    public function run()
    {
        call_user_func([$this, 'a-b']);
    }
}
