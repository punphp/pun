<?php

namespace TraitPrototypeControls;

trait CyclicRequirement
{
    use CyclicRequirement;

    abstract public function check(int $value): void;
}

class Cyclic
{
    use CyclicRequirement;

    public function check(string $value): void {}
}
