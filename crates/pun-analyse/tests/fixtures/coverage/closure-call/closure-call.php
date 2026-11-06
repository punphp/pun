<?php

namespace ClosureCall;

interface Input
{
}

class Runner
{
    private int $definition = 1;

    private function fixParallel(): int
    {
        return $this->definition;
    }
}

class FixCode
{
    public function __construct(private Input $input)
    {
    }

    public function run(bool $parallel): void
    {
        $method = $parallel ? 'fixParallel' : 'fixSequential';
        (fn () => $this->{$method}())->call(new Runner());
        (function () {
            return $this->nope();
        })->call(new Runner());
        (fn () => $this->definition)->call($this->input);
        \PHPStan\dumpType((fn () => $this)->call(new Runner()));
        \PHPStan\dumpType((fn () => $this->definition)->call(new Runner()));
    }
}
