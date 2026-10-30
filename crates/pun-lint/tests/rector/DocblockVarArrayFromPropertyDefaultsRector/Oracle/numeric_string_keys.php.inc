<?php

class SomeClass
{
    private array $items = ['1' => 'a', '01' => 'b'];
}
-----
<?php

class SomeClass
{
    /**
     * @var array<int|string, string>
     */
    private array $items = ['1' => 'a', '01' => 'b'];
}
