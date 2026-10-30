<?php

namespace App;

abstract class Shape {}
final class Circle extends Shape {}
final class Square extends Shape {}

final class OwnClasses
{
    public function shapes(): array
    {
        return [new Circle(), new Square()];
    }
}
-----
<?php

namespace App;

abstract class Shape {}
final class Circle extends Shape {}
final class Square extends Shape {}

final class OwnClasses
{
    /**
     * @return \App\Shape[]
     */
    public function shapes(): array
    {
        return [new Circle(), new Square()];
    }
}
