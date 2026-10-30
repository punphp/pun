<?php
namespace MixedAliasOriginal;

trait Body { public function OriginalMethod(string $old): void {} }
class Base { use Body { OriginalMethod as MixedAlias; } }
class Child extends Base { public function mixedalias(string $new): void {} }
function calls(Base $base): void {
    $base->ORIGINALMETHOD(old: 'no original override exists in this file');
    $base->originalmethod(old: 123);
}
class OwnMethod {
    use Body { OriginalMethod as MixedAlias; }
    public function mixedalias(int $own): void {}
}
function own(OwnMethod $owner): void { $owner->MIXEDALIAS(own: 'own declaration takes precedence'); }
