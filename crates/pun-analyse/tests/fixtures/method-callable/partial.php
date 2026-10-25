<?php
namespace MethodCallablePartial;
class Subject
{
    public function instance(int $value): void {}
    public static function statik(int $value): void {}
    public function create(): void
    {
        $this->missing(?);
        $this->missing(?, ...);
        $this->missing(1, ...);
        self::missing(?);
        self::missing(?, ...);
        self::missing(1, ...);
        1 |> $this->instance(...);
        1 |> self::statik(...);
    }
}
