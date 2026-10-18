<?php

namespace Anon;

interface Shape
{
    public function area(): int;
}

abstract class Base
{
    abstract public function name(): string;
}

function make()
{
    return new class {
        protected function hidden(): void
        {
        }
    };
}

function pair(): array
{
    return [new class implements Shape {}, new class extends Base {}];
}

function args(): object
{
    return new class (1, 2) {
        private function twice(): void
        {
        }

        public function call(): void
        {
            $this->twice();
            $this->missing();
        }
    };
}

function constructed(): object
{
    return new class () {
        public function __construct(int $a)
        {
        }
    };
}

function greeter(): object
{
    return new class {
        use Greets;
    };
}
