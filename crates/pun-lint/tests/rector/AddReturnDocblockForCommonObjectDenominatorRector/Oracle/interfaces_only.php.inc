<?php

namespace App;

interface Shape {}
interface Named {}
final class Circle implements Shape, Named {}
final class Square implements Shape, Named {}

final class SomeClass
{
    public function run(): array
    {
        return [new Circle(), new Square()];
    }
}
-----
<?php

namespace App;

interface Shape {}
interface Named {}
final class Circle implements Shape, Named {}
final class Square implements Shape, Named {}

final class SomeClass
{
    /**
     * @return \App\Shape[]
     */
    public function run(): array
    {
        return [new Circle(), new Square()];
    }
}
