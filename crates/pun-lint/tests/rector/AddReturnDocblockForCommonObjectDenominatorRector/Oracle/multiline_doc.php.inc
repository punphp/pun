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
     * @return array<
     *     int
     * >
     */
    public function run(): array
    {
        return [new Circle(), new Square()];
    }
}
