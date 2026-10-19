<?php

class Plain
{
    public function run()
    {
        echo 1;
    }
}

class A
{
    public function __sleep()
    {
        echo 1;
        return ['a'];
    }

    public function run()
    {
        call_user_func([...$this->items, 'go']);
    }
}
