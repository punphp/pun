<?php

namespace Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture;

use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\AbstractType;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\DifferentInterface;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\NewInterface;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeSubscriber;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeTrait;

interface SubDifferent extends DifferentInterface
{
    public function renameMe();
}

interface SubNew extends NewInterface
{
    public function some_old();
}

class Impl implements DifferentInterface
{
    public function renameMe() {}
    public function renameMe2() {}
}

class Type1 extends AbstractType
{
    public function SETDEFAULTOPTIONS() {}
}

abstract class Type2 extends AbstractType
{
    abstract public function setDefaultOptions();
}

class ParentUses
{
    use SomeTrait;
}

class GrandUses extends ParentUses
{
    public function _test() {}
}

trait AnyTrait
{
    public function notify() {}
    public function _test() {}
    public function old() {}
}

trait TraitHasNew
{
    public function old() {}
    public function new() {}
}

class SubscriberKid extends SomeSubscriber
{
    public function old(): int { return 1; }
}

enum OwnEnum
{
    public function oldEnumMethod() {}
}
-----
<?php

namespace Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Fixture;

use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\AbstractType;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\DifferentInterface;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\NewInterface;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeSubscriber;
use Rector\Tests\Renaming\Rector\MethodCall\RenameMethodRector\Source\SomeTrait;

interface SubDifferent extends DifferentInterface
{
    public function toNewVersion();
}

interface SubNew extends NewInterface
{
    public function some_new();
}

class Impl implements DifferentInterface
{
    public function toNewVersion() {}
    public function renameMe2() {}
}

class Type1 extends AbstractType
{
    public function configureOptions() {}
}

abstract class Type2 extends AbstractType
{
    abstract public function configureOptions();
}

class ParentUses
{
    use SomeTrait;
}

class GrandUses extends ParentUses
{
    public function test() {}
}

trait AnyTrait
{
    public function notify() {}
    public function _test() {}
    public function old() {}
}

trait TraitHasNew
{
    public function old() {}
    public function new() {}
}

class SubscriberKid extends SomeSubscriber
{
    public function new(): int { return 1; }
}

enum OwnEnum
{
    public function oldEnumMethod() {}
}
