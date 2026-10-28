<?php
namespace MixedAliasDirect;

trait Body { public function OriginalMethod(string $old): void {} }
class Base { use Body { OriginalMethod as MixedAlias; } }
class Child extends Base { public function mixedalias(string $new): void {} }
function calls(Base $base): void {
    $base->MixedAlias(old: 'written case');
    $base->mixedalias(old: 'lowercase lookup');
    $base->MIXEDALIAS(old: 'uppercase lookup');
    $base->mIxEdAlIaS(old: 123);
    $base->OriginalMethod(old: 'an alias override does not rename the original');
}
class InvalidChild extends Base { public function MIXEDALIAS(int $old): void {} }
