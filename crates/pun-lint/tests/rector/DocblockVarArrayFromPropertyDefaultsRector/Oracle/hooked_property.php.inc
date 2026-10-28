<?php

class SomeClass
{
    public array $items = ['a'] {
        set => $value;
    }
}
-----
<?php

class SomeClass
{
    /**
     * @var string[]
     */
    public array $items = ['a'] {
        set => $value;
    }
}
