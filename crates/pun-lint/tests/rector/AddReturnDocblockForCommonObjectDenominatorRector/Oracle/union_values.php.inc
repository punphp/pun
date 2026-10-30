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
        return [rand(0, 1) ? new Circle() : new Square(), new Square()];
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
     * @return \App\Base[]
     */
    public function run(): array
    {
        return [rand(0, 1) ? new Circle() : new Square(), new Square()];
    }
}
