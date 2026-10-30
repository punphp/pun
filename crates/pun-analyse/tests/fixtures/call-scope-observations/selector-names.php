<?php
class ObservationSelectorSink {
    public function run($value) {}
    public function other($value) {}
}
/**
 * @param 'run'|'other' $literalUnion
 * @param 'run'|int $mixedUnion
 * @param callable-string $callbackName
 * @param class-string<ObservationSelectorSink> $className
 */
function observationSelectorNames(
    ObservationSelectorSink $sink,
    $literalUnion,
    $mixedUnion,
    string $unknown,
    string $callbackName,
    string $className,
): void {
    $sink->{'run'}(1); // observe
    $sink->$literalUnion(2); // observe
    $sink->$mixedUnion(3); // observe
    $sink->$unknown(4); // observe
    $sink->$callbackName(5); // observe
    $sink->$className(6); // observe
}
/** @template TName of 'run'
 * @param TName $name
 */
function observationLiteralTemplate(ObservationSelectorSink $sink, $name): void {
    $sink->$name(7); // observe
}
/** @template TName of ('run'|'other')
 * @param TName $name
 */
function observationUnionTemplate(ObservationSelectorSink $sink, $name): void {
    $sink->$name(8); // observe
}
/** @template TName of string
 * @param TName&'run' $name
 */
function observationIntersectionTemplate(ObservationSelectorSink $sink, $name): void {
    $sink->$name(9); // observe
}
