<?php
namespace MixedAliasVisibility;

trait Body { protected function OriginalMethod(string $old): void {} }
class Base { use Body { OriginalMethod as public; } }
class Child extends Base { public function originalmethod(string $new): void {} }
class AliasBase { use Body { OriginalMethod as public PublicAlias; } }
class AliasChild extends AliasBase { public function publicalias(string $alias): void {} }
function calls(Base $base, AliasBase $alias): void {
    $base->ORIGINALMETHOD(old: 'visibility-only adaptation does not rename the method');
    $alias->PUBLICALIAS(old: 'named adaptation keeps written alias spelling');
    $base->originalmethod(old: 123);
}
trait PrivateBody { public function OriginalMethod(string $old): void {} }
class PrivateOwner { use PrivateBody { OriginalMethod as private PrivateAlias; } }
class PrivateChild extends PrivateOwner { public function privatealias(string $new): void {} }
function privateCall(PrivateOwner $owner): void {
    $owner->PRIVATEALIAS(old: 'private alias is not a rename prototype');
}
