<?php

namespace App;

class Box
{
    public Box $known;
}

class Magic
{
    public function __get(string $name): mixed
    {
        return null;
    }
}

/**
 * @property Box $tagged
 */
class Tagged extends Magic
{
}

/**
 * @property Box $tagged
 */
class TaggedStatic
{
}

/**
 * @property Box $tagged
 */
#[\AllowDynamicProperties]
class Dynamic
{
}

/**
 * @mixin Box
 */
class WithMixin extends Magic
{
}

/**
 * @mixin Box|Magic
 */
class Either extends Magic
{
}

/**
 * @mixin Second
 */
class First extends Magic
{
}

/**
 * @mixin First
 */
class Second extends Magic
{
}

/**
 * @template T
 * @mixin T
 */
class Generic extends Magic
{
}

/**
 * @phpstan-require-extends Magic
 */
interface Required
{
}

/**
 * @phpstan-require-extends Box
 */
interface RequiredBox
{
}

enum Suit
{
    case Hearts;
}

/**
 * @param-closure-this Magic $closure
 */
function test(\Closure $closure): void
{
}

/**
 * @param Generic<Box> $generic
 */
// each value is a variable of its own: Mago types a variable passed to an unknown function (`tap()` here) anew
function run(Box $box, Box $other, Magic $magic, Tagged $tagged, TaggedStatic $static, Dynamic $dynamic, WithMixin $mixin, Either $either, First $first, $generic, Required $required, \stdClass $object, \stdClass $crate, Box $known, RequiredBox $requiredBox)
{
    // a property PHPStan does not find has the type `*ERROR*`, which takes the parameter's type away
    tap($box->missing, function (Box $x) {
    });
    tap($magic->missing, function (Box $x) {
    });
    tap($other->missing, function (mixed $x) {
    });
    // a `@property` tag and a mixin count only for a class that allows dynamic properties
    tap($static->tagged, function (Magic $x) {
    });
    tap($tagged->tagged, function (Magic $x) {
    });
    tap($dynamic->tagged, function (Magic $x) {
    });
    tap($mixin->known, function (Magic $x) {
    });
    tap($either->known, function (Magic $x) {
    });
    tap($first->missing, function (Box $x) {
    });
    tap($generic->known, function (Magic $x) {
    });
    tap($required->missing, function (Box $x) {
    });
    tap($requiredBox->known, function (Magic $x) {
    });
    tap($object->missing, function (Box $x) {
    });
    tap($crate->missing, function (mixed $x) {
    });
    tap($known->known, function (Magic $x) {
    });
    tap(Suit::Hearts->name, function (int $x) {
    });
}

test(function () {
    tap($this->missing, function (Box $x) {
    });
});
-----
<?php

namespace App;

class Box
{
    public Box $known;
}

class Magic
{
    public function __get(string $name): mixed
    {
        return null;
    }
}

/**
 * @property Box $tagged
 */
class Tagged extends Magic
{
}

/**
 * @property Box $tagged
 */
class TaggedStatic
{
}

/**
 * @property Box $tagged
 */
#[\AllowDynamicProperties]
class Dynamic
{
}

/**
 * @mixin Box
 */
class WithMixin extends Magic
{
}

/**
 * @mixin Box|Magic
 */
class Either extends Magic
{
}

/**
 * @mixin Second
 */
class First extends Magic
{
}

/**
 * @mixin First
 */
class Second extends Magic
{
}

/**
 * @template T
 * @mixin T
 */
class Generic extends Magic
{
}

/**
 * @phpstan-require-extends Magic
 */
interface Required
{
}

/**
 * @phpstan-require-extends Box
 */
interface RequiredBox
{
}

enum Suit
{
    case Hearts;
}

/**
 * @param-closure-this Magic $closure
 */
function test(\Closure $closure): void
{
}

/**
 * @param Generic<Box> $generic
 */
// each value is a variable of its own: Mago types a variable passed to an unknown function (`tap()` here) anew
function run(Box $box, Box $other, Magic $magic, Tagged $tagged, TaggedStatic $static, Dynamic $dynamic, WithMixin $mixin, Either $either, First $first, $generic, Required $required, \stdClass $object, \stdClass $crate, Box $known, RequiredBox $requiredBox)
{
    // a property PHPStan does not find has the type `*ERROR*`, which takes the parameter's type away
    tap($box->missing, function ($x) {
    });
    tap($magic->missing, function ($x) {
    });
    tap($other->missing, function ($x) {
    });
    // a `@property` tag and a mixin count only for a class that allows dynamic properties
    tap($static->tagged, function ($x) {
    });
    tap($tagged->tagged, function (\App\Box $x) {
    });
    tap($dynamic->tagged, function (\App\Box $x) {
    });
    tap($mixin->known, function (\App\Box $x) {
    });
    tap($either->known, function ($x) {
    });
    tap($first->missing, function ($x) {
    });
    tap($generic->known, function (\App\Box $x) {
    });
    tap($required->missing, function ($x) {
    });
    tap($requiredBox->known, function (\App\Box $x) {
    });
    tap($object->missing, function ($x) {
    });
    tap($crate->missing, function ($x) {
    });
    tap($known->known, function (\App\Box $x) {
    });
    tap(Suit::Hearts->name, function (string $x) {
    });
}

test(function () {
    tap($this->missing, function ($x) {
    });
});
