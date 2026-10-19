<?php

namespace App;

interface HasRun
{
    public function run();
}

class Magic
{
    public function __call($name, $args)
    {
    }
}

class Pseudo
{
    public static function pseudo()
    {
    }
}

class Base
{
    public function base()
    {
    }
}

class Middle extends Base
{
}

class Lonely
{
    public function a()
    {
        parent::a();
        parent::b();
        // keep
        PARENT::c();
        if (true) {
            parent::d();
        }
        $x = parent::e();
        $y .= parent::f();
        $z = (parent::g());
        parent::{'h'}();
        return parent::i();
    }

    abstract public function j();
}

class FromBase extends Middle implements HasRun
{
    public function a()
    {
        parent::base();
        parent::run();
        parent::missing();
        $x = parent::missing();
    }
}

class FromMagic extends Magic
{
    public function a()
    {
        parent::anything();
    }
}

class FromPseudo extends Pseudo
{
    public function a()
    {
        parent::pseudo();
        parent::other();
    }
}

class Vendor extends \Some\Vendor\Base
{
    public function a()
    {
        parent::anything();
    }
}

trait T
{
    public function a()
    {
        parent::a();
    }
}

new class {
    public function a()
    {
        parent::a();
    }
};
-----
<?php

namespace App;

interface HasRun
{
    public function run();
}

class Magic
{
    public function __call($name, $args)
    {
    }
}

class Pseudo
{
    public static function pseudo()
    {
    }
}

class Base
{
    public function base()
    {
    }
}

class Middle extends Base
{
}

class Lonely
{
    public function a()
    {
        if (true) {
            parent::d();
        }
        $x = null;
        $y .= parent::f();
        $z = (null);
        parent::{'h'}();
        return parent::i();
    }

    abstract public function j();
}

class FromBase extends Middle implements HasRun
{
    public function a()
    {
        parent::base();
        parent::run();
        $x = null;
    }
}

class FromMagic extends Magic
{
    public function a()
    {
        parent::anything();
    }
}

class FromPseudo extends Pseudo
{
    public function a()
    {
        parent::pseudo();
    }
}

class Vendor extends \Some\Vendor\Base
{
    public function a()
    {
        parent::anything();
    }
}

trait T
{
    public function a()
    {
        parent::a();
    }
}

new class {
    public function a()
    {
        parent::a();
    }
};
