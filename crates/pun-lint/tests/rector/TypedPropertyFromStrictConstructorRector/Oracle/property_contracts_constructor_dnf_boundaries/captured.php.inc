<?php

namespace QualityPropertyConstructorDnfBoundaries;

interface LeftSide {}
interface RightSide {}

final class ConstructorDnfBoundaries
{
    private $nullableIntersection;
    private $dnf;

    public function __construct((LeftSide&RightSide)|null $nullableIntersection, (LeftSide&RightSide)|int $dnf)
    {
        $this->nullableIntersection = $nullableIntersection;
        $this->dnf = $dnf;
    }
}
