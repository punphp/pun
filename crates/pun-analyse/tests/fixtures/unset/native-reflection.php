<?php

namespace UnsetNativeReflection;

/**
 * @property int $native
 * @property int $documented
 * @property int $virtual
 * @property int $backed
 * @property int $ordinary
 * @property int $final
 * @property int $documentedFinal
 * @property int $privateSet
 */
class Overlay
{
    public readonly int $native;
    /** @readonly */
    public int $documented = 1;
    public int $virtual { get => 1; }
    public int $backed { set { $this->backed = $value; } }
    public int $ordinary = 1;
    final public int $final = 1;
    /** @final */
    public int $documentedFinal = 1;
    public private(set) int $privateSet = 1;

    public function __construct() { $this->native = 1; }
}

function overlays(Overlay $o): void
{
    unset($o->native);
    unset($o->documented);
    unset($o->virtual);
    unset($o->backed);
    unset($o->ordinary);
    unset($o->final);
    unset($o->documentedFinal);
    unset($o->privateSet);
}

class ParentProperties
{
    public readonly int $native;
    public int $hooked { get => 1; }
    final public int $final = 1;
    public int $ordinary = 1;
    public function __construct() { $this->native = 1; }
}

/**
 * @property int $native
 * @property int $hooked
 * @property int $final
 * @property int $ordinary
 */
class ChildOverlay extends ParentProperties {}

/**
 * @final
 * @property int $ordinary
 */
class FinalOverlay extends ParentProperties {}

function inherited(ChildOverlay $child, FinalOverlay $final): void
{
    unset($child->native);
    unset($child->hooked);
    unset($child->final);
    unset($child->ordinary);
    unset($final->ordinary);
}

trait NativeProperties
{
    public readonly int $native;
    public int $hooked { get => 1; }
    public function __construct() { $this->native = 1; }
}

/** @property int $native
 * @property int $hooked
 */
class TraitOverlay { use NativeProperties; }

class InheritsTraitOverlay extends TraitOverlay {}

function traits(TraitOverlay $own, InheritsTraitOverlay $inherited): void
{
    unset($own->native);
    unset($own->hooked);
    unset($inherited->native);
    unset($inherited->hooked);
}

/** @property int $native */
readonly class ReadonlyOverlay
{
    public int $native;
    public function __construct() { $this->native = 1; }
}

/** @property-read int $native
 * @property int $absent
 */
#[\AllowDynamicProperties]
class AnnotationOnly {}

function controls(ReadonlyOverlay $readonly, AnnotationOnly $magic): void
{
    unset($readonly->native);
    unset($magic->native);
    unset($magic->absent);
}

/** @phpstan-readonly */
class PhpstanReadonly { public int $p = 1; }
/** @psalm-readonly */
class PsalmReadonly { public int $p = 1; }
/** @phan-read-only */
class PhanReadonly { public int $p = 1; }
/** @phpstan-readonly-allow-private-mutation */
class PhpstanPrivateMutation { public int $p = 1; }
/** @psalm-readonly-allow-private-mutation */
class PsalmPrivateMutation { public int $p = 1; }

class InheritsReadonlyAlias extends PhpstanReadonly { public int $own = 1; }

function aliases(PhpstanReadonly $a, PsalmReadonly $b, PhanReadonly $c, PhpstanPrivateMutation $d, PsalmPrivateMutation $e, InheritsReadonlyAlias $child): void
{
    unset($a->p);
    unset($b->p);
    unset($c->p);
    unset($d->p);
    unset($e->p);
    unset($child->p);
    unset($child->own);
}

#[\Attribute(\Attribute::TARGET_PROPERTY | \Attribute::TARGET_CLASS | \Attribute::TARGET_PARAMETER)]
class Marker { public function __construct(public string $text) {} }

class NativeModifiers
{
    public readonly int $first, $second;
    #[Marker('readonly')]
    public readonly /* comment */ int $commented;
    public readonly // comment
        int $lineComment;

    public function __construct(
        #[Marker('readonly')] public readonly /* comment */ int $promoted = 1,
    ) {
        $this->first = $this->second = $this->commented = $this->lineComment = 1;
    }
}

/** @readonly */
#[Marker('readonly')]
class MisleadingText
{
    #[Marker('readonly')]
    public int $attribute = 1;
    public /* readonly */ int $comment = 1;
    public // readonly
        int $lineComment = 1;
    public int $first = 1, $second = 2;
}

#[Marker('readonly')]
class OrdinaryText
{
    #[Marker('readonly')]
    public /* readonly */ int $p = 1;
}

function modifiers(NativeModifiers $native, MisleadingText $documented, OrdinaryText $ordinary): void
{
    unset($native->first);
    unset($native->second);
    unset($native->commented);
    unset($native->lineComment);
    unset($native->promoted);
    unset($documented->attribute);
    unset($documented->comment);
    unset($documented->lineComment);
    unset($documented->first);
    unset($documented->second);
    unset($ordinary->p);
}

function anonymousReadonly(): object
{
    return new readonly class {
        public int $native;
        public function __construct() { $this->native = 1; }
        public function test(): void { unset($this->native); }
    };
}
