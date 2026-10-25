<?php
namespace RequiredReceivers;

class RequiredMethodParent
{
    public function accepting(string $value): void {}
    public static function staticAccepting(string $value): void {}
}
/** @phpstan-require-extends RequiredMethodParent */
interface RequiresMethodParent {}
interface InheritedRequirement extends RequiresMethodParent {}
interface UnrelatedInterface {}
class UnrelatedClass {}

function requiredMethod(RequiresMethodParent $object): void
{
    $object->accepting(123);
    $object->accepting('valid');
    $object->missing();
    $object::staticAccepting(123);
    $object::staticAccepting('valid');
    $object::missing();
    $object->missing(...);
    $object::missing(...);
    \PHPStan\dumpType($object);
}

function explicitIntersection(RequiresMethodParent&RequiredMethodParent $object): void
{
    $object->accepting(123);
    $object->missing();
    \PHPStan\dumpType($object);
}

function unrelatedIntersection(RequiresMethodParent&UnrelatedInterface $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

function unrelatedClass(UnrelatedClass&UnrelatedInterface $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

function inherited(InheritedRequirement $object): void
{
    $object->accepting('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}

interface RequiredImplementation { public function implemented(string $value): void; }
/** @phpstan-require-implements RequiredImplementation */
trait RequiresImplementation {}
class ImplementationUser { use RequiresImplementation; }
function requireImplements(ImplementationUser $object): void
{
    $object->implemented('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}

/** @template T */
class GenericParent { /** @param T $value */ public function generic($value): void {} }
/** @phpstan-require-extends GenericParent<string> */
interface GenericRequirement {}
function genericRequirement(GenericRequirement $object): void
{
    $object->generic(123);
    $object->generic('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}
/** @param GenericParent<string>&GenericRequirement $object */
function genericIntersection($object): void
{
    $object->generic(123);
    $object->generic('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}

class lowerparent {}
/** @phpstan-require-extends lowerparent */
interface lowerrequirement {}
function lowercaseIntersection(lowerrequirement&lowerparent $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}
function reverseIntersection(RequiredMethodParent&RequiresMethodParent $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

/** @param class-string<RequiresMethodParent> $class */
function classString($class): void
{
    $class::missing();
    $class::missing(...);
    \PHPStan\dumpType($class);
}

/** @template T of RequiresMethodParent
 * @param T $object
 */
function templateRequirement($object): void
{
    $object->accepting('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}

function returned(): RequiresMethodParent { throw new \RuntimeException(); }
function returnedReceiver(): void
{
    returned()->missing();
    returned()::missing();
    \PHPStan\dumpType(returned());
}

function branches(RequiresMethodParent $object, bool $flag): void
{
    if ($flag) { $object->missing(); }
    $object->missing();
    if ($object instanceof RequiredMethodParent) {
        $object->missing();
        \PHPStan\dumpType($object);
    }
}

interface GrandContract { public function ancestor(string $value): void; }
interface ParentContract extends GrandContract {}
class ParentWithContract implements ParentContract
{
    public function ancestor(string $value): void {}
}
/** @phpstan-require-extends ParentWithContract */
interface RequirementWithContract {}
function explicitContract(RequirementWithContract&GrandContract $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

/** @phpstan-require-implements ParentContract */
interface InvalidImplementationRequirement {}
function invalidImplementationRequirement(InvalidImplementationRequirement $object): void
{
    $object->ancestor('valid');
    $object->missing();
    \PHPStan\dumpType($object);
}

interface DiamondLeft extends GrandContract {}
interface DiamondRight extends GrandContract {}
interface Diamond extends DiamondLeft, DiamondRight {}
function nominalIntersection(Diamond&GrandContract $object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

/** @extends GenericParent<string> */
class GenericChild extends GenericParent {}
/** @param GenericChild&GenericParent<string> $object */
function nominalGenericIntersection($object): void
{
    $object->missing();
    \PHPStan\dumpType($object);
}

function requiredCounts(RequiresMethodParent $object): void
{
    $object->accepting();
    $object->accepting(...);
    $object::staticAccepting();
    $object::staticAccepting(...);
}

class NarrowedChild extends RequiredMethodParent {}
function narrowedRequirement(RequiresMethodParent $object): void
{
    if ($object instanceof NarrowedChild) {
        $object->missing();
        \PHPStan\dumpType($object);
    }
}
