<?php
namespace MethodCallableGuards;
class Open
{
    public function create(): void
    {
        if (is_callable([$this, 'Maybe'])) { $this->Maybe(...); }
        $this->Maybe(...);
        if (!is_callable([$this, 'Other'])) { $this->Other(...); }
        if (is_callable([$this, 'Maybe'])) { $this->Different(...); }
        is_callable([$this, 'AndGuard']) && $this->AndGuard(...);
        if (is_callable('strlen')) { $this->NotAnArray(...); }
        if (is_callable([$this])) { $this->NotAPair(...); }
        $method = random_int(0, 1) ? 'First' : 'Second';
        if (is_callable([$this, $method])) { $this->Unrelated(...); }
        if (!is_callable([$this, 'Early'])) { return; }
        $this->Early(...);
    }
}
final class Closed
{
    public function create(): void
    {
        if (is_callable([$this, 'Missing'])) { $this->Missing(...); }
        if (is_callable([$this, 'MISSING'])) { $this->missing(...); }
        if (method_exists($this, 'Missing')) { $this->Missing(...); }
    }
}
final class ClosedMagic
{
    public function __call(string $name, array $arguments): mixed { return null; }
    public static function __callStatic(string $name, array $arguments): mixed { return null; }
    public function create(): void
    {
        if (is_callable([$this, 'Missing'])) { $this->Missing(...); }
        if (is_callable([self::class, 'Missing'])) { self::Missing(...); }
    }
}
function create(Open $open, Closed $closed, int $integer): void
{
    if (is_callable([$open, 'Maybe'])) { $open->Maybe(...); }
    if (is_callable([$closed, 'Missing'])) { $closed->Missing(...); }
    if (is_callable([$integer, 'Missing'])) { $integer->Missing(...); }
    if (is_callable([Closed::class, 'MISSING'])) { Closed::missing(...); }
}
