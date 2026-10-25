<?php
namespace AttributeContracts;

#[\Attribute(\Attribute::TARGET_ALL | \Attribute::IS_REPEATABLE)]
class Arguments
{
    /** @no-named-arguments */
    public function __construct(int $number, string &$text) {}
}

// Level zero does not check argument types, references, extra arguments, or @no-named-arguments.
#[Arguments('wrong', 123)]
#[Arguments(1, 'valid', 'extra')]
#[Arguments(text: 'valid', number: 1)]
class Accepted {}

#[Arguments(number: 1)]
class OnClass {}

#[Arguments(number: 1)]
function onFunction(): void {}

$closure = #[Arguments(number: 1)] function (): void {};
$arrow = #[Arguments(number: 1)] fn (): int => 1;

function onParameter(#[Arguments(number: 1)] int $value): void {}

class Members
{
    #[Arguments(number: 1)]
    public const C = 1;
    #[Arguments(number: 1)]
    public int $property = 1;
    #[Arguments(number: 1)]
    public function method(): void {}
    public int $hook {
        #[Arguments(number: 1)]
        get => 1;
    }
    public function __construct(#[Arguments(number: 1)] public int $promoted) {}
}

enum Cases
{
    #[Arguments(number: 1)]
    case C;
}

#[Arguments(number: 1)]
const C = 1;

#[Arguments(number: 1)]
trait OnTrait {}
class UsesTrait { use OnTrait; }

#[Arguments]
#[Arguments(1)]
#[Arguments(unknown: 1)]
#[Arguments(1, text: 'valid', number: 2)]
class InvalidArguments {}

// Native reflection retains the last value of a repeated named argument, before ArgumentsNormalizer.
#[\Attribute(flags: \Attribute::TARGET_METHOD, flags: \Attribute::TARGET_CLASS)]
class DuplicateFlags {}
#[DuplicateFlags]
class UsesDuplicateFlags {}

// A positional argument after a named flag is not the constructor's first argument.
#[\Attribute(flags: \Attribute::TARGET_METHOD, \Attribute::TARGET_CLASS)]
class NamedFirst {}
#[NamedFirst]
class UsesNamedFirst {}

#[\Attribute(\Attribute::TARGET_METHOD, flags: \Attribute::TARGET_CLASS)]
class PositionalFirst {}
#[PositionalFirst]
class UsesPositionalFirst {}

#[\Attribute(foo: \Attribute::TARGET_METHOD, \Attribute::TARGET_CLASS)]
class DefaultHole {}
#[DefaultHole]
class UsesDefaultHole {}

#[\Attribute(foo: 1, bar: 2, 4)]
class MissingHole {}
#[MissingHole]
class UsesMissingHole {}

#[\Attribute(foo: \Attribute::TARGET_METHOD, foo: \Attribute::TARGET_CLASS)]
class DuplicateUnknown {}
#[DuplicateUnknown]
class UsesDuplicateUnknown {}

#[\Attribute(1, 2, 4)]
class ExtraPositional {}
#[ExtraPositional]
class UsesExtraPositional {}

class Hooks
{
    public int $separate {
        #[\NoDiscard]
        #[\NoDiscard]
        get => 1;
    }
    public int $together {
        #[\NoDiscard, \NoDiscard]
        get => 1;
    }
    public int $mixed {
        #[\NoDiscard, \NoDiscard]
        #[\NoDiscard]
        get => 1;
    }
}

// Filling $flags' optional hole must use TARGET_ALL, not the later positional argument's TARGET_CLASS.
#[DefaultHole]
function defaultHoleSupportsFunctions(): void {}
#[ExtraPositional]
function extraPositionalKeepsFirstFlag(): void {}

#[\Attribute(foo: \Attribute::TARGET_METHOD, bar: \Attribute::TARGET_PROPERTY, foo: \Attribute::TARGET_CLASS)]
class DuplicateKeepsFirstPosition {}
#[DuplicateKeepsFirstPosition]
class UsesDuplicateKeepsFirstPosition {}
