<?php
namespace RenamedNamedArgument;

interface ShortContract { public function method(string $original): void; }
interface LongContract { public function method(string $original, string $extra = ''): void; }
interface VariadicContract { public function method(string ...$original): void; }
class ShortChild implements ShortContract { public function method(string $renamed): void {} }
class LongChild implements LongContract { public function method(string $renamed, string $extra = ''): void {} }
class VariadicChild implements VariadicContract { public function method(string ...$renamed): void {} }
/** @no-named-arguments */
interface NoNames { public function method(string $original): void; }
interface DocNoNames { /** @no-named-arguments */ public function method(string $original): void; }
class NoNamesChild implements NoNames { public function method(string $renamed): void {} }
class DocNoNamesChild implements DocNoNames { public function method(string $renamed): void {} }
class DocNoNamesGrandchild extends DocNoNamesChild { public function method(string $again): void {} }
trait NoNamesTrait { /** @no-named-arguments */ abstract public function method(string $original): void; }
class NoNamesTraitChild {
    use NoNamesTrait;
    public function method(string $renamed): void {}
}
class NoNamesTraitGrandchild extends NoNamesTraitChild { public function method(string $again): void {} }
interface Root { public function method(string $original): void; }
interface Left extends Root {}
interface Right extends Root {}
interface Diamond extends Left, Right {}
class DiamondChild implements Diamond { public function method(string $renamed): void {} }
class IteratorChild extends \ArrayIterator {
    public function seek(int $renamed): void {}
}
/** @template T of ShortContract */
class Generic {
    /** @param T $object */
    public function use($object): void { $object->method(original: 'template receiver'); }
}
/**
 * @template TName of 'method'
 * @param TName $name
 */
function genericName(ShortContract $object, string $name): void {
    $object->$name(original: 'template method name');
}
function intersections(
    ShortContract&LongContract $long,
    LongContract&VariadicContract $variadic,
    ShortContract&NoNames $mixedNames,
    ShortContract|NoNames $unionNoNames,
    DocNoNamesGrandchild $doc,
    NoNamesTraitChild $trait,
    Diamond $diamond,
    \ArrayIterator $builtin,
): void {
    $long->method(original: 'longest signature');
    $variadic->method(original: 'variadic wins');
    $mixedNames->method(original: 'intersection accepts if any member does');
    $unionNoNames->method(original: 'union accepts only if all do');
    $doc->method(again: 'inherited method tag');
    $trait->method(renamed: 'abstract trait PHPDoc inherited');
    $diamond->method(original: 'diamond');
    $builtin->seek(offset: 0);
}
/**
 * @param array{value?: string} $optional
 * @param array{0?: string} $optionalPositional
 * @param array{value: string}|array{second: int} $alternatives
 * @param array<string, string> $unknown
 */
function unpacking(Contract $object, array $optional, array $optionalPositional, array $alternatives, array $unknown): void {
    $known = ['value' => 'variable constant array'];
    $object->run(...$known);
    $object->run(...$optional);
    $object->run(...$optionalPositional);
    $object->run(...$alternatives);
    $object->run(...$unknown);
    $object->run(...['0' => 'numeric string key', 'value' => 'named duplicate']);
    $object->run(...[]);
    $object->run(...$optionalPositional, value: 'optional skipped');
}
