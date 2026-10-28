<?php declare(strict_types=1);

namespace ReflectedNamedPolicy;

class AEnabledBase
{
    public function run(int $value): void {}
}

class AEnabledChild extends AEnabledBase
{
    public function run(int $renamed): void {}
}

trait TaggedSource
{
    /** @no-named-arguments */
    public function run(int $value): void {}
}

class ZAliasedBase
{
    use TaggedSource { run as alias; }
}

// FileTypeMapper gives the tagged trait PHPDoc to the alias, not to its original name.
// The union's first declaring class owns the rename; its second member controls collection policy only.
function aliasedOriginal(AEnabledBase|ZAliasedBase $object): void
{
    $object->run(value: 1);
}

function directAlias(ZAliasedBase $object): void
{
    $object->run(value: 1);
    $object->alias(value: 1);
    $object->alias(1);
}

trait PlainSource
{
    public function run(int $value): void {}
}

class PlainAlias
{
    use PlainSource { run as alias; }
}

function plainAlias(PlainAlias $object): void
{
    $object->run(value: 1);
    $object->alias(value: 1);
}

class OwnTagged
{
    /** @no-named-arguments */
    public function run(int $value): void {}
}

class OwnTaggedChild extends OwnTagged
{
    public function run(int $renamed): void {}
}

function ownMethods(AEnabledBase $enabled, OwnTagged $tagged, OwnTaggedChild $child): void
{
    $enabled->run(value: 1);
    $enabled->run(1);
    $tagged->run(value: 1);
    $child->run(renamed: 1);
}

/** @no-named-arguments */
class TaggedParent
{
    public function run(int $value): void {}
}

class InheritsTaggedParent extends TaggedParent {}

class OverridesTaggedParent extends TaggedParent
{
    public function run(int $value): void {}
}

function parentClassTags(InheritsTaggedParent $inherited, OverridesTaggedParent $own): void
{
    $inherited->run(value: 1);
    $own->run(value: 1);
}

interface MethodTaggedContract
{
    /** @no-named-arguments */
    public function run(int $value): void;
}

class MethodTaggedImplementation implements MethodTaggedContract
{
    public function run(int $value): void {}
}

/** @no-named-arguments */
interface ClassTaggedContract
{
    public function run(int $value): void;
}

class ClassTaggedImplementation implements ClassTaggedContract
{
    public function run(int $value): void {}
}

function interfaceTags(
    MethodTaggedContract $methodContract,
    MethodTaggedImplementation $methodImplementation,
    ClassTaggedContract $classContract,
    ClassTaggedImplementation $classImplementation,
): void {
    $methodContract->run(value: 1);
    $methodImplementation->run(value: 1);
    $classContract->run(value: 1);
    $classImplementation->run(value: 1);
}

/** @no-named-arguments */
class TaggedConsumer
{
    use PlainSource { run as alias; }
}

/** @no-named-arguments */
trait ClassTaggedSource
{
    public function run(int $value): void {}
}

class UntaggedConsumer
{
    use ClassTaggedSource { run as alias; }
}

function consumingClassTags(TaggedConsumer $tagged, UntaggedConsumer $untagged): void
{
    $tagged->run(value: 1);
    $tagged->alias(value: 1);
    $untagged->run(value: 1);
    $untagged->alias(value: 1);
}
