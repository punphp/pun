<?php

namespace Pun\Vm;

trait Helps
{
    public function persist(object $entity): void
    {
    }
}

class Unrelated
{
    public function persist(object $entity): void
    {
    }
}

interface Returns
{
    public function persist(object $entity): int;
}
