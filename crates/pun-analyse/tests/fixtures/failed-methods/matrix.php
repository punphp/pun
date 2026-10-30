<?php
namespace FailedMethodsProbe;
class Known {
    public function value(): int { return 31; }
    private function hidden(): string { return 'hidden'; }
    public static function staticValue(): string { return 'static'; }
}
function missingNamed(Unknown $receiver): void {
    \PHPStan\dumpType($receiver->value());
    $copy = $receiver->value();
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType((new Known)->value());
}
function missingMember(Known $receiver): void {
    \PHPStan\dumpType($receiver->absent());
    \PHPStan\dumpType($receiver->hidden());
    \PHPStan\dumpType($receiver->value());
}
function nonObjects(int $integer, ?Known $nullable): void {
    \PHPStan\dumpType($integer->value());
    \PHPStan\dumpType((null)->value());
    \PHPStan\dumpType($nullable?->value());
    \PHPStan\dumpType($nullable->value());
}
function unknownAndKnown(Unknown|Known $receiver): void {
    \PHPStan\dumpType($receiver->value());
}
function nonObjectAndKnown(int|Known $receiver): void {
    \PHPStan\dumpType($receiver->value());
}
function generalObject(object $receiver): void {
    \PHPStan\dumpType($receiver->value());
}
function generalMixed(mixed $receiver): void {
    \PHPStan\dumpType($receiver->value());
}
function staticCalls(): void {
    \PHPStan\dumpType(Unknown::value());
    \PHPStan\dumpType(Known::absent());
    \PHPStan\dumpType(Known::staticValue());
}
