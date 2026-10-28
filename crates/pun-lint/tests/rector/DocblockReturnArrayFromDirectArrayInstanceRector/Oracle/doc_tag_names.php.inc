<?php

final class SomeClass
{
    /**
     * @psalm-return array
     * @ORM\Column(type="json")
     * @\Foo\Bar
     * @_foo
     * @a:b
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @psalm-return array
     * @ORM\Column(type="json")
     * @\Foo\Bar
     * @_foo
     * @a:b
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
