<?php
namespace RenamedNamedArgument;

class Renamed implements Contract {
    public function run(string $renamed, int $other = 0): void {}
}

class Stable implements Contract {
    public function run(string $value, int $second = 0): void {}
}

class OtherRenamed implements OtherContract {
    public function run(string $otherValue, int $second = 0): void {}
}
