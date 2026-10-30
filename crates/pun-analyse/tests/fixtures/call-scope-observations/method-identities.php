<?php
trait IdentityMethods {
    /** @phpstan-impure */
    public function traitMutation(): int { return 1; }
    /** @phpstan-impure */
    public static function traitStatic(): int { return 1; }
    public function boot($v = null) {}
    public function render(): string { return ''; }
}
class IdentityConsumer {
    use IdentityMethods {
        traitMutation as aliasMutation;
        traitStatic as aliasStatic;
        boot as __construct;
        render as __toString;
    }
}
class IdentityChild extends IdentityConsumer {}
function identityCalls(IdentityChild $o, bool $b): void {
    $o->traitMutation(); // observe
    $o->aliasMutation(); // observe
    $o->ALIASMutation(); // observe
    $o?->aliasMutation();
    $m = $b ? 'traitMutation' : 'aliasMutation';
    $o->$m(); // observe
    IdentityChild::traitStatic();
    IdentityChild::aliasStatic();
    $n = $b ? 'traitStatic' : 'aliasStatic';
    IdentityChild::$n();
    new IdentityChild();
    (string) $o;
}
