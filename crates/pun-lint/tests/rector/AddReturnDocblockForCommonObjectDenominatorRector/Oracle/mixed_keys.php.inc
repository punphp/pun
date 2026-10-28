<?php

namespace App;

interface Shape {}
abstract class Base implements Shape {}
final class Circle extends Base {}
final class Square extends Base {}
final class Other {}

final class SomeClass
{
    public function run(): array
    {
        return ['a' => new Circle(), 5 => new Square()];
    }
}
-----
<?php

namespace App;

interface Shape {}
abstract class Base implements Shape {}
final class Circle extends Base {}
final class Square extends Base {}
final class Other {}

final class SomeClass
{
    /**
     * @return array<string|int, \App\Base>
     */
    public function run(): array
    {
        return ['a' => new Circle(), 5 => new Square()];
    }
}
