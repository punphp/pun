<?php
namespace ExactConstructorObjects;

class ZContract { public function run(string $value): void {} }
class AContract { public function run(string $value): void {} }
class ZRenamed extends ZContract { public function run(string $zValue): void {} }
class ARenamed extends AContract { public function run(string $aValue): void {} }
class Inherited extends ZContract {}

function declared(ZContract|AContract $forward, AContract|ZContract $reverse): void {
    rand();
    $forward->run(value: 'written forward');
    $reverse->run(value: 'written reverse');
}

function assigned(AContract|ZContract $forward, ZContract|AContract $reverse, bool $flag): void {
    $forward = $flag ? new ZContract() : new AContract();
    $reverse = $flag ? new AContract() : new ZContract();
    $copy = $forward;
    rand();
    $forward->run(value: 'assigned exact forward');
    $reverse->run(value: 'assigned exact reverse');
    $copy->run(value: 'copied exact union');
    $same = $flag ? new ZContract() : new ZContract();
    $same->run(value: 'same exact class');
    if ($flag) {
        $joined = new ZContract();
    } else {
        $joined = new AContract();
    }
    $joined->run(value: 'joined exact union');
    if ($flag) {
        $opposite = new AContract();
    } else {
        $opposite = new ZContract();
    }
    $opposite->run(value: 'opposite exact join');
}

function broad(ZContract $parameter, bool $flag): void {
    $forward = $flag ? $parameter : new ZRenamed();
    $reverse = $flag ? new ZRenamed() : $parameter;
    rand();
    $forward->run(value: 'broad parent absorbs exact child');
    $reverse->run(value: 'reversed broad parent');
    $value = new ZContract();
    $value = $parameter;
    $value->run(value: 'reassigned broad');
    $value = new ZContract();
    $value->run(value: 'reassigned exact');
}

function inherited(bool $flag): void {
    $value = new Inherited();
    $value->run(value: 'parent declaring reflection remains broad');
    $either = $flag ? new Inherited() : new ZContract();
    $either->run(value: 'inherited first member');
    $reverse = $flag ? new ZContract() : new Inherited();
    $reverse->run(value: 'own first member');
}

/** @template T */
class Box {
    /** @param T $item */
    public function __construct($item) {}
    public function run(string $value): void {}
}
/** @extends Box<int> */
class RenamedBox extends Box { public function run(string $boxValue): void {} }

function generic(bool $flag): void {
    $first = new Box(1);
    $second = new Box(2);
    $value = $flag ? $first : $second;
    $copy = $value;
    rand();
    $copy->run(value: 'exact generic own declaration');
}

class Receiver {
    public function run(string $value): void {}
    public function calls(bool $flag): void {
        $self = new self();
        $late = new static();
        $self->run(value: 'exact self');
        $late->run(value: 'late static is not exact');
        $this->run(value: 'this remains a distinct receiver');
        $union = $flag ? new self() : new static();
        $union->run(value: 'self with static');
    }
}
class RenamedReceiver extends Receiver { public function run(string $renamed): void {} }

/** @template T */
class EmptyBox { public function run(string $value): void {} }
class RenamedEmptyBox extends EmptyBox { public function run(string $emptyValue): void {} }
/**
 * @template T
 * @extends Box<T>
 */
class ChildBox extends Box { public function run(string $value): void {} }
class RenamedChildBox extends ChildBox { public function run(string $childValue): void {} }
final class FinalOwn { public function run(string $value): void {} }
abstract class AbstractOwn { public function run(string $value): void {} }
class RenamedAbstractOwn extends AbstractOwn { public function run(string $abstractValue): void {} }

function constructor_reflections(): void {
    (new Box(1))->run(value: 'own generic constructor traverses class templates');
    (new EmptyBox())->run(value: 'dummy generic constructor remains exact');
    (new ChildBox(1))->run(value: 'inherited generic constructor with own method remains exact');
    (new FinalOwn())->run(value: 'native finality needs no override');
    (new AbstractOwn())->run(value: 'asFinal does not override abstract declarations');
}

/** @template T */
class Holding {
    /** @param T $item */
    public function __construct(private $item) {}
    /** @return T */
    public function value() { return $this->item; }
}

/**
 * @template T
 * @param T $value
 * @return T
 */
function identity($value) { return $value; }

function inferred_receivers(): void {
    $holder = new Holding(new ZContract());
    $holder->value()->run(value: 'constructor template inference removes the override');
    identity(new ZContract())->run(value: 'function template inference removes the override');
}
