<?php
namespace RenamedAliasPolicy;

class Ancestor {
    /** @no-named-arguments */
    public function alias(string $old): void {}
}
trait Body {
    public function original(string $old): void {}
}
class Owner extends Ancestor {
    use Body { original as alias; }
}
class Child extends Owner {
    public function alias(string $new): void {}
}
function inheritedPolicy(Owner $owner): void {
    $owner->alias(old: 'alias inherits its own prototype policy');
    $owner->original(old: 'the original has a different policy and no override');
}

/** @no-named-arguments */
class ForbiddenOwner {
    use Body { original as alias; }
}
class ForbiddenChild extends ForbiddenOwner {
    public function alias(string $new): void {}
}
function classPolicy(ForbiddenOwner $owner): void {
    $owner->alias(old: 'the declaring class still forbids names');
}

trait DocumentedBody {
    /** @no-named-arguments */
    public function source(string $old): void {}
}
class DocumentedOwner {
    use DocumentedBody { source as alias; }
}
class DocumentedChild extends DocumentedOwner {
    public function alias(string $new): void {}
}
function methodPolicy(DocumentedOwner $owner): void {
    $owner->alias(old: 'the alias retains the source method policy');
}
