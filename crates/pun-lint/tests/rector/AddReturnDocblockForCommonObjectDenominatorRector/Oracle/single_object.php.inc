<?php

final class Circle {}

final class SomeClass
{
    public function run(): array
    {
        return [new Circle()];
    }
}
-----
<?php

final class Circle {}

final class SomeClass
{
    /**
     * @return \Circle[]
     */
    public function run(): array
    {
        return [new Circle()];
    }
}
