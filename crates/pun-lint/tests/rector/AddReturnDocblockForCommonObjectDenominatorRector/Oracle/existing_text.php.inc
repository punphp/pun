<?php

namespace App;

interface Animal {}
final class Dog implements Animal {}
final class Cat implements Animal {}

final class ExistingText
{
    /**
     * The animals.
     */
    public function animals(): array
    {
        return [new Dog(), new Cat()];
    }
}
-----
<?php

namespace App;

interface Animal {}
final class Dog implements Animal {}
final class Cat implements Animal {}

final class ExistingText
{
    /**
     * The animals.
     * @return \App\Animal[]
     */
    public function animals(): array
    {
        return [new Dog(), new Cat()];
    }
}
