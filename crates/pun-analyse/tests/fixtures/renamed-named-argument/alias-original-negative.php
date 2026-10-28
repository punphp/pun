<?php
namespace RenamedAliasOriginal;

trait Body {
    public function original(string $old): void {}
}
class Base {
    use Body { original as alias; }
}
class Child extends Base {
    public function alias(string $new): void {}
}
function calls(Base $base): void {
    $base->original(old: 'an alias override must not rename the original');
}

class OriginalChild extends Base {
    public function original(string $renamed): void {}
}
function originalOverride(Base $base): void {
    $base->original(old: 'an independent original override still joins');
}

final class FinalOwner {
    use Body { original as alias; }
}
class InvalidFinalChild extends FinalOwner {
    public function alias(string $new): void {}
}
function finalOwner(FinalOwner $owner): void {
    $owner->alias(old: 'a final declaring class never contributes a call');
}
