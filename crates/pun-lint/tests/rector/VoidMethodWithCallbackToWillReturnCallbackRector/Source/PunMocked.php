<?php

namespace Pun\Vm;

interface Persister
{
    public function persist(object $entity): void;
}

class Manager
{
    public const SOME = 'x';

    public function persist(object $entity): void
    {
    }

    public function count(string $name): int
    {
        return 1;
    }

    public function maybe(): ?string
    {
        return null;
    }
}

final class FinalManager
{
    public function persist(object $entity): void
    {
    }
}

enum Kind
{
    case A;

    public function persist(): void
    {
    }
}

abstract class Base extends Manager
{
}

class Child extends Base
{
}
