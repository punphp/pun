<?php

namespace Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture;

use Nette\Utils\Html;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\Enum\SomeEnumWithMethod;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\Foo;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeSubscriber;

class CustomType
{
    public function notify() {}

    public function run()
    {
        $this->notify();
        self::notify();
        static::notify();
        $this->NOTIFY();
    }
}

class KidType extends CustomType
{
    public function run()
    {
        parent::notify();
        CustomType::notify();
        \Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture\CustomType::notify();
    }
}

final class KidWithInvoke extends CustomType
{
    public function __invoke() {}
}

class Sub extends SomeSubscriber
{
}

final class SubWithNew extends SomeSubscriber
{
    public function new() {}
}

final class SubOwnOld extends SomeSubscriber
{
    public function old(): int { return 1; }
    public function new() {}
}

trait NewTrait
{
    public function old(): int { return 2; }
    public function new() {}
}

final class SubTrait extends SomeSubscriber
{
    use NewTrait;
}

function f(?CustomType $a, CustomType|Foo $b, KidType $c, KidWithInvoke $d, Html $h, ?Html $n, Sub $s, SubWithNew $sn, SubOwnOld $so, SubTrait $st, \ReflectionMethod $rm, \ReflectionNamedType $nt, CustomType|false $g, $m, Foo $foo)
{
    $a->notify();
    $a?->notify();
    $b->notify();
    $c->notify();
    $d->notify();
    $h->add('x');
    $n?->add('x');
    $h->addToArray('x');
    $x = $h->addToArray('x')['k'];
    $s->old();
    $sn->old();
    $so->old();
    $st->old();
    $rm->getTentativeReturnType();
    $nt->getTentativeReturnType();
    $g->notify();
    $a->$m();
    $foo->old();
    SomeEnumWithMethod::FIRST->oldEnumMethod();
    $e = SomeEnumWithMethod::FIRST;
    $e->oldEnumMethod();
}
-----
<?php

namespace Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture;

use Nette\Utils\Html;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\Enum\SomeEnumWithMethod;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\Foo;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeSubscriber;

class CustomType
{
    public function __invoke() {}

    public function run()
    {
        $this->__invoke();
        self::__invoke();
        static::__invoke();
        $this->__invoke();
    }
}

class KidType extends CustomType
{
    public function run()
    {
        parent::__invoke();
        CustomType::__invoke();
        \Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture\CustomType::__invoke();
    }
}

final class KidWithInvoke extends CustomType
{
    public function __invoke() {}
}

class Sub extends SomeSubscriber
{
}

final class SubWithNew extends SomeSubscriber
{
    public function new() {}
}

final class SubOwnOld extends SomeSubscriber
{
    public function old(): int { return 1; }
    public function new() {}
}

trait NewTrait
{
    public function old(): int { return 2; }
    public function new() {}
}

final class SubTrait extends SomeSubscriber
{
    use NewTrait;
}

function f(?CustomType $a, CustomType|Foo $b, KidType $c, KidWithInvoke $d, Html $h, ?Html $n, Sub $s, SubWithNew $sn, SubOwnOld $so, SubTrait $st, \ReflectionMethod $rm, \ReflectionNamedType $nt, CustomType|false $g, $m, Foo $foo)
{
    $a->__invoke();
    $a?->__invoke();
    $b->notify();
    $c->__invoke();
    $d->__invoke();
    $h->addHtml('x');
    $n?->addHtml('x');
    $h->addToHtmlArray('x')['hey'];
    $x = $h->addToHtmlArray('x')['hey']['k'];
    $s->new();
    $sn->new();
    $so->old();
    $st->old();
    $rm->getReturnType();
    $nt->getTentativeReturnType();
    $g->__invoke();
    $a->$m();
    $foo->new();
    SomeEnumWithMethod::FIRST->newEnumMethod();
    $e = SomeEnumWithMethod::FIRST;
    $e->newEnumMethod();
}
