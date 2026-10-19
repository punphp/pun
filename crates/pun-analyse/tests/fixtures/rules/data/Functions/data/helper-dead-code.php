<?php

namespace HelperDeadCode;

use function print_r as output;
use function define as declareConstant;
use function compact as gather;

function outputChecks(bool $flag, array $arguments): void
{
    $x = output('value', false);
    $x = \var_export(value: 'value', return: false);
    $x = \highlight_string(return: false, string: '<?php');
    $x = print_r('value', $flag);
    $x = print_r(...$arguments);
    $x = print_r();
    $x = print_r(return: false);
    $x = print_r('value', true);
    $x = print_r(value: 'value', unknown: true);
    $x = print_r(value: 'value', unknown: false);
    $x = print_r(unknown: 'value');
    print_r('value');
    $x = missing_output('value');
    \DEFINE('HELPER_CONST', true, false);
    $dynamic = 'print_r';
    $x = $dynamic('value');
}

function captures(string $unknown): void
{
    $used = 1;
    $unused = 2;
    $name = 'used';
    $nested = fn () => function () use ($used, $unused) { echo $used; };
    $f = function () use ($used, $unused) { $g = fn () => $used; };
    $f = function () use ($used, $unused) { echo \compact('used'); };
    $f = function () use ($used, $unused) { echo compact(['used']); };
    $f = function () use ($used, $unused, $name) { echo $$name; };
    $f = function () use ($used, $unused) { echo ${'used'}; };
    $f = function () use ($used, $unused, $unknown) { echo $$unknown; };
    $f = function () use ($used, $unused) { echo \func_get_args(); };
    $f = function () use ($used, $unused) { echo get_defined_vars(); };
    $f = function () use ($used, $unused) { require 'missing.php'; };
    $f = function () use ($used, $unused) { require_once 'missing.php'; };
    $f = function () use ($used, $unused) { include_once 'missing.php'; };
    $f = function () use ($used, $unused) { eval(''); };
    $f = function () use ($used, $unused) { $used = 3; };
    $f = function () use ($used, $unused) { $dynamic = 'compact'; $dynamic('used'); };
}

interface Contract { public function __construct($contract); }
class ImplementsContract implements Contract { public function __construct($contract) {} }
#[\Attribute]
class AttributeClass { public function __construct($attribute) {} }
#[\Attribute(flags: \Attribute::TARGET_METHOD)]
class NamedAttributeClass { public function __construct($attribute) {} }
#[\Attribute(\Attribute::TARGET_CLASS)]
class PositionalAttributeClass { public function __construct($attribute) {} }
#[\Attribute('invalid')]
class InvalidAttributeClass { public function __construct($unused) {} }
#[\Attribute, \Attribute]
class DuplicateAttributeClass { public function __construct($unused) {} }
abstract class AbstractConstructor { abstract public function __construct($parameter); }
class Promoted { public function __construct(public int $promoted, int $unused) {} }
$anonymous = new class(1) { public function __construct($unused) {} };

class ParentClass
{
    #[\NoDiscard]
    public static function value(): int { return 1; }
    #[\NoDiscard]
    public function instance(): int { return 1; }
    public function allowed(): int { return 2; }
}
class ChildClass extends ParentClass
{
    public function __construct($used, $unused) { echo $used; }
    public function test(): void
    {
        $this->instance();
        self::value();
        static::value();
        parent::value();
        (void) $this->allowed();
    }
}
class Other { public function instance(): int { return 2; } }

function methods(ParentClass|Other $both, ParentClass|int $partial, ?ParentClass $nullable, object $object, mixed $mixed, string $method): void
{
    $both->instance();
    $partial->instance();
    $nullable->instance();
    $nullable?->instance();
    $object->instance();
    $mixed->instance();
    $nullable->$method();
    $nullable::value();
    ParentClass::missing();
    MissingClass::value();
    $date = new \DateTimeImmutable();
    $date->modify('+1 day');
    (void) $date->modify('+1 day');
    $date = new \DateTime();
    (void) $date->modify('+1 day');
}

function unionMethod(ParentClass|Other $object): void
{
    (void) $object->instance();
}

/** @param class-string<ParentClass> $class */
function classString(string $class): void
{
    $class::value();
}

function aliases(): void
{
    declareConstant('HELPER_ALIAS_CONST', true, false);
    $used = 1;
    $unused = 2;
    $f = function () use ($used, $unused) { echo gather('used'); };
}

enum Result
{
    case Value;
    #[\NoDiscard]
    public function value(): int { return 1; }
}
function enumResult(Result $result): void
{
    $result->value();
    Result::Value->value();
}

class NoMethods {}
function missingMethod(ParentClass|NoMethods $partial, UnknownReceiver $missing): void
{
    $partial->instance();
    $missing->instance();
}
function nestedClosure(): void
{
    $used = 1;
    $unused = 2;
    $f = function () use ($used, $unused) {
        $g = function () { echo $unused; };
        isset($used);
        empty($used);
    };
}

interface Marker {}
interface Required {
    #[\NoDiscard]
    public function value(): int;
}
interface Optional { public function value(int $optional = 0): int; }
function intersections(Marker&Required $a, Required&Optional $b): void
{
    $a->value();
    $b->value();
    (void) $a->value();
    (void) $b->value();
}

class Alpha {
    #[\NoDiscard]
    public static function value(): int { return 1; }
}
class Beta {
    #[\NoDiscard]
    public function value(): int { return 2; }
}
function combinedMethod(Alpha|Beta $value): void
{
    $value->VaLuE();
}

namespace HelperDeadCode\Imports;

use function strlen as define;

function userAlias(): void
{
    define('not-a-constant', true, false);
}
