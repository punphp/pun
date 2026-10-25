<?php
namespace RenamedNamedArgument;

interface Contract {
    public function run(string $value, int $second = 0): void;
}

interface OtherContract {
    public function run(string $value, int $second = 0): void;
}

interface DifferentContract {
    public function run(string $different): void;
}
