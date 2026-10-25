<?php
namespace MethodCallableNullsafe;
class Target
{
    public function Required(int $value): void {}
    public static function RequiredStatic(int $value): void {}
}
class Holder
{
    public Target $target;
    public ?Target $nullable;
    public function target(): Target { return new Target; }
    public function nullable(): ?Target { return null; }
}
function create(?Holder $holder, Holder $present): void
{
    ($holder?->target)->Required(...);
    ($holder?->nullable)->Required(...);
    ($holder?->target())->Required(...);
    ($holder?->nullable())->Required(...);
    ($holder?->target)::RequiredStatic(...);
    ($holder?->nullable)::RequiredStatic(...);
    ($present->target)->Missing(...);
    ($present->nullable)->Required(...);
    $copy = $holder?->target;
    $copy->Required(...);
    $copy::RequiredStatic(...);
    $selector = 'Missing';
    $holder->{$selector}(...);
    $holder::{$selector}(...);
}
