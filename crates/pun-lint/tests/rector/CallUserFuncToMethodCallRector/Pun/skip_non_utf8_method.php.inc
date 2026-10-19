<?php

// Rector prints the method name's bytes after `->`, here no UTF-8; Pun leaves the call.
class A
{
    public function run()
    {
        call_user_func([$this, "\xff"]);
    }
}
