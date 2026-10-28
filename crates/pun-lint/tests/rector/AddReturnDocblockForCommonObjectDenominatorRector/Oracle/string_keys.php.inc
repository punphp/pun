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
        return ['a' => new Circle(), 'b' => new Square()];
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
     * @return array<string, \App\Base>
     */
    public function run(): array
    {
        return ['a' => new Circle(), 'b' => new Square()];
    }
}
