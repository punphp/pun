<?php

namespace ParentIntersections;

interface Contract {}
interface OtherContract {}
class Base implements Contract {}
class Unrelated {}
trait Marker {}

function outside(parent&Contract $value): parent&Contract { return $value; }
function outsideClass(parent&Base $value): parent&Base { return $value; }
function outsideDnf((parent&Contract)|null $value): (parent&Contract)|null { return $value; }
function outsideImpossible(parent&int $value): parent&int { throw new \RuntimeException(); }
function outsideNullableImpossible((parent&int)|null $value): (parent&int)|null { return null; }

$closure = function (parent&Contract $value): parent&Contract { return $value; };
$arrow = fn (parent&Contract $value): parent&Contract => $value;
$nullableClosure = function ((parent&Contract)|null $value): (parent&Contract)|null { return $value; };

class NoParent
{
    public function interfaceType(parent&Contract $value): parent&Contract { return $value; }
    public function classType(parent&Base $value): parent&Base { return $value; }
    public function nullable((parent&Contract)|null $value): (parent&Contract)|null { return $value; }
    public function impossible(parent&int $value): parent&int { throw new \RuntimeException(); }

    public function closures(): void
    {
        $closure = function (parent&Contract $value): parent&Contract { return $value; };
        $arrow = fn ((parent&Contract)|null $value): (parent&Contract)|null => $value;
    }
}

class MissingParent extends Missing
{
    public function interfaceType(PaReNt&Contract $value): PaReNt&Contract { return $value; }
    public function nullable((PaReNt&Contract)|null $value): (PaReNt&Contract)|null { return $value; }
}

interface ChildContract extends Contract
{
    public function noClassParent(parent&OtherContract $value): parent&OtherContract;
}

final class HasParent extends Base
{
    public function implemented(parent&Contract $value): parent&Contract { return $value; }
    public function possible(parent&OtherContract $value): parent&OtherContract { return $value; }
    public function same(parent&Base $value): parent&Base { return $value; }
    public function impossibleClass(parent&Unrelated $value): parent&Unrelated { throw new \RuntimeException(); }
    public function impossibleTrait(parent&Marker $value): parent&Marker { throw new \RuntimeException(); }
    public function impossibleScalar(parent&int $value): parent&int { throw new \RuntimeException(); }
    public function nullable((parent&OtherContract)|null $value): (parent&OtherContract)|null { return $value; }
    public function nullableImpossible((parent&Unrelated)|null $value): (parent&Unrelated)|null { return null; }

    public function closures(): void
    {
        $closure = function (parent&OtherContract $value): parent&OtherContract { return $value; };
        $arrow = fn ((parent&Contract)|null $value): (parent&Contract)|null => $value;
        $impossible = function (parent&Unrelated $value): parent&Unrelated { throw new \RuntimeException(); };
    }
}

class BuiltinParent extends \Exception
{
    public function implemented(parent&\Throwable $value): parent&\Throwable { return $value; }
    public function possible(parent&Contract $value): parent&Contract { return $value; }
    public function impossible(parent&\stdClass $value): parent&\stdClass { throw new \RuntimeException(); }
}

final class FinalBase {}
final class FinalContract implements Contract {}

class HasFinalParent extends FinalBase
{
    public function impossible(parent&Contract $value): parent&Contract { throw new \RuntimeException(); }
    public function nullableImpossible((parent&Contract)|null $value): (parent&Contract)|null { return null; }
}

class HasFinalContractParent extends FinalContract
{
    public function implemented(parent&Contract $value): parent&Contract { return $value; }
}
