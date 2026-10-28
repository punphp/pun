<?php

namespace Pun\Entity;

/** @Entity */
// a line comment after the doc
class LineAfterDoc
{
}

/** @Document */
/* a block comment */
class BlockAfterDoc
{
}

/* @Entity */
class NotADoc
{
}

/** @ORM\Document */
#[\Attribute]
/** another doc */
final class AttributeBetween
{
    public function setIsActive(bool $a): void {}
    public function setActive(bool $a): void {}
}

/**
 * @EntityListeners
 */
abstract class AbstractEntity
{
}

/** @Entity */
interface EntityInterface
{
}

/** @Entity */
trait EntityTrait
{
}

/** @Entity */
enum EntityEnum
{
}

/** @entity */
class LowerCase
{
}

/**
 * Not one.
 */
class Plain
{
    public function setName(string $n): void {}
}

/** @Entity */
class WithMagic
{
    public function __call($n, $a) {}
}

/**
 * @Entity
 * @method void setIsReady(bool $r)
 */
class WithMethodTag
{
}

/** @Entity */
class Child extends Plain
{
    public function isVisible(): bool {}
}

class ChildNoDoc extends LineAfterDoc
{
}

/**
 * @EntityListeners
 */
class Listeners
{
}

/**
 * @method void setIsDone(bool $d)
 */
class ParentWithTag
{
    public function setIsFresh(bool $f): void {}
}

/** @Entity */
class FromParent extends ParentWithTag
{
}
