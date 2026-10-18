<?php

class LocalAssign
{
    public function __construct(private int $a)
    {
        $b = $a;
    }
}

class DynamicName
{
    public function __construct(private int $a)
    {
        $this->{$a} = $a;
    }
}

class OtherObject
{
    public function __construct(private int $a)
    {
        $this->b->a = $a;
    }
}

class VariableName
{
    public function __construct(private int $a)
    {
        $this->$a = $a;
    }
}

class NestedName
{
    public function __construct(private int $a)
    {
        $this->$$a = $a;
    }
}

class StringName
{
    public function __construct(private int $a)
    {
        $this->{'a'} = $a;
    }
}
-----
<?php

class LocalAssign
{
    public function __construct(private int $a)
    {
        $b = $a;
    }
}

class DynamicName
{
    public function __construct(private int $a)
    {
    }
}

class OtherObject
{
    public function __construct(private int $a)
    {
        $this->b->a = $a;
    }
}

class VariableName
{
    public function __construct(private int $a)
    {
    }
}

class NestedName
{
    public function __construct(private int $a)
    {
        $this->$$a = $a;
    }
}

class StringName
{
    public function __construct(private int $a)
    {
        $this->{'a'} = $a;
    }
}
