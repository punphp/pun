<?php

namespace Oracle\PropertyExists2;

/**
 * @mixin Target
 */
class WithMixin
{
}

class Target
{
    public $mixed;
}

class Holder
{
    public static $counter;

    public int|string|null $union = null;

    public ?int $five = 5;

    public function __construct(
        public ?int $required,
        public ?int $withFive = 5,
        public readonly ?int $ro = null,
    ) {
    }

    public function run(WithMixin $mixin, Pure $pure, self $same, object $any, $untyped)
    {
        isset($this->counter);
        isset($mixin->mixed);
        isset($this->union);
        isset($this->five);
        isset($this->required);
        isset($this->withFive);
        isset($this->ro);
        isset($pure->value);
        isset($pure->name);
        isset($same->union);
        isset($any->x);
        isset($untyped->x);
        isset($this->{'union'});
        isset(($this)->union);
        $f = function () {
            return isset($this->union);
        };
        $g = static function () {
            return isset($this->union);
        };
        $h = new class {
            public $inner;

            public function check()
            {
                return isset($this->inner) && isset($this->missing);
            }
        };
    }
}

enum Pure
{
    case A;
}
-----
<?php

namespace Oracle\PropertyExists2;

/**
 * @mixin Target
 */
class WithMixin
{
}

class Target
{
    public $mixed;
}

class Holder
{
    public static $counter;

    public int|string|null $union = null;

    public ?int $five = 5;

    public function __construct(
        public ?int $required,
        public ?int $withFive = 5,
        public readonly ?int $ro = null,
    ) {
    }

    public function run(WithMixin $mixin, Pure $pure, self $same, object $any, $untyped)
    {
        property_exists($this, 'counter') && $this->counter !== null;
        property_exists($mixin, 'mixed') && $mixin->mixed !== null;
        $this->union !== null;
        isset($this->five);
        isset($this->required);
        isset($this->withFive);
        $this->ro !== null;
        property_exists($pure, 'value') && $pure->value !== null;
        $pure->name !== null;
        $same->union !== null;
        isset($any->x);
        isset($untyped->x);
        isset($this->{'union'});
        ($this)->union !== null;
        $f = function () {
            return $this->union !== null;
        };
        $g = static function () {
            return isset($this->union);
        };
        $h = new class {
            public $inner;

            public function check()
            {
                return $this->inner !== null && (property_exists($this, 'missing') && $this->missing !== null);
            }
        };
    }
}

enum Pure
{
    case A;
}
