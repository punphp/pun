<?php

namespace PunFunctionCallableNullsafe;

class Holder
{
    public \Closure $yes;
    public ?\Closure $maybe;
    public int $number;
    public ?int $nullableNumber;
    public function yes(): \Closure { return $this->yes; }
    public function maybe(): ?\Closure { return $this->maybe; }
    public function number(): int { return $this->number; }
    public function nullableNumber(): ?int { return $this->nullableNumber; }
}

function callables(?Holder $holder): void
{
    ($holder?->yes)(...);
    ($holder?->maybe)(...);
    ($holder?->number)(...);
    ($holder?->nullableNumber)(...);
    ($holder?->yes())(...);
    ($holder?->maybe())(...);
    ($holder?->number())(...);
    ($holder?->nullableNumber())(...);
    $saved = $holder?->yes;
    $saved(...);
    $null = null;
    ($null?->yes())(...);
}
