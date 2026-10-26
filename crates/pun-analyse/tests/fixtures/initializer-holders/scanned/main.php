<?php
namespace ScannedHolders;
\PHPStan\dumpType(HOLDERS);
\PHPStan\dumpType(REFLECTED);
\PHPStan\dumpType(Values::VALUE);
\PHPStan\dumpType(FORWARD);
const FORWARD = (true ? A::class : B::class)::V;
\PHPStan\dumpType(FORWARD);
class LocalValues { public const VALUE = (true ? A::class : B::class)::V; }
\PHPStan\dumpType(LocalValues::VALUE);
#[EitherClass] class AcceptedAttribute {}
#[DifferentTargets] class RejectedAttribute {}
