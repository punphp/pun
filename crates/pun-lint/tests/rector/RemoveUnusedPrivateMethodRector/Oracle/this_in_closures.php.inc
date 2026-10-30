<?php

namespace App;

final class ThisInClosures
{
    public function run(): array
    {
        $a = function () {
            return $this->fromClosure();
        };
        $b = fn () => $this->fromArrow();
        $c = static function () {
            return $this->fromStaticClosure();
        };
        $d = static fn () => $this->fromStaticArrow();

        return [$a, $b, $c, $d];
    }

    public static function make(): int
    {
        return $this->fromStaticMethod();
    }

    private function fromClosure(): int
    {
        return 1;
    }

    private function fromArrow(): int
    {
        return 2;
    }

    private function fromStaticClosure(): int
    {
        return 3;
    }

    private function fromStaticArrow(): int
    {
        return 4;
    }

    private function fromStaticMethod(): int
    {
        return 5;
    }
}
-----
<?php

namespace App;

final class ThisInClosures
{
    public function run(): array
    {
        $a = function () {
            return $this->fromClosure();
        };
        $b = fn () => $this->fromArrow();
        $c = static function () {
            return $this->fromStaticClosure();
        };
        $d = static fn () => $this->fromStaticArrow();

        return [$a, $b, $c, $d];
    }

    public static function make(): int
    {
        return $this->fromStaticMethod();
    }

    private function fromClosure(): int
    {
        return 1;
    }

    private function fromArrow(): int
    {
        return 2;
    }
}
