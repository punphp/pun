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
-----
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
    public function __serialize(): array
    {
        echo 1;
        return ['a' => $this->a];
    }

    public function run()
    {
        call_user_func([...$this->items, 'go']);
    }
}
