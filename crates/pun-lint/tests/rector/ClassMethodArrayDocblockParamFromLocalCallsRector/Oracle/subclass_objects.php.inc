<?php

class Animal {}
class Dog extends Animal {}
class Cat extends Animal {}

class SomeClass
{
    public function go(): void
    {
        $this->run(new Dog());
        $this->run(new Cat());
        $this->run(new Animal());
    }

    private function run(array|Animal $items): void
    {
    }
}
