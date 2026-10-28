<?php
namespace RenamedAliasPrivate;

trait PublicBody {
    public function original(string $old): void {}
}
class PrivateOwner {
    use PublicBody { original as private hidden; }
    public function caller(): void {
        $this->hidden(old: 'a private alias cannot dispatch to a child');
    }
}
class PrivateChild extends PrivateOwner {
    public function hidden(string $new): void {}
}
function original(PrivateOwner $owner): void {
    $owner->original(old: 'a private alias override is not an original override');
}

trait PrivateBody {
    private function source(string $old): void {}
}
class PublicOwner {
    use PrivateBody { source as public exposed; }
}
class PublicChild extends PublicOwner {
    public function exposed(string $new): void {}
}
function exposed(PublicOwner $owner): void {
    $owner->exposed(old: 'a public alias of a private source is overridable');
}

class ProtectedOwner {
    use PrivateBody { source as protected exposed; }
    public function caller(): void {
        $this->exposed(old: 'a protected alias of a private source is overridable');
    }
}
class ProtectedChild extends ProtectedOwner {
    public function exposed(string $new): void {}
}
