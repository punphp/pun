<?php
namespace MixedAliasInherited;

trait Body { public function OriginalMethod(string $old): void {} }
trait Inner { use Body { OriginalMethod as InnerAlias; } }
trait Outer { use Inner; }
class Base { use Outer; }
class Middle extends Base {}
class Child extends Middle { public function inneralias(string $new): void {} }
class Realiased { use Outer { InnerAlias as OtherAlias; } }
class RealiasedChild extends Realiased { public function otheralias(string $renamed): void {} }
function calls(Middle $middle, Realiased $realiased): void {
    $middle->INNERALIAS(old: 'inherited through traits and parent');
    $middle->InnerAlias(old: 123);
    $realiased->OTHERALIAS(old: 'the nearest alias supplies its written spelling');
    $realiased->InnerAlias(old: 'outer alias override must not rename its source alias');
    $realiased->OriginalMethod(old: 'nor its source declaration');
}
