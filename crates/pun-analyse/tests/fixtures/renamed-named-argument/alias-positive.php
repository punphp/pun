<?php
namespace RenamedAliasPositive;

trait Body {
    public function original(string $old): void {}
}
class Base {
    use Body { original as alias; }
}
class Child extends Base {
    public function alias(string $new): void {}
}
class Middle extends Base {}
class Descendant extends Middle {
    public function alias(string $inherited): void {}
}
function calls(Base $base, Middle $middle): void {
    $base->alias(old: 'direct alias');
    $middle->alias(old: 'inherited alias still belongs to Base');
}
