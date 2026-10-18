<?php

namespace TraitContext;

trait Greets
{
    public function hello(): void
    {
        missing_in_hello();
        if ($this instanceof Greets) {
            return;
        }
        missing_after_trait_check();
        $this->undefinedAfterTraitCheck();
    }

    public function shadowed(MissingInShadowed $x): void
    {
    }

    public function renamed(MissingInRenamed $x): void
    {
    }

    public function hidden(MissingInHidden $x): void
    {
    }
}

trait Other
{
    public function other(MissingInOther $x): void
    {
        if ($this instanceof Base) {
            return;
        }
        missing_after_base_check();
        $this->undefinedAfterBaseCheck();
    }

    public function hello(): void
    {
        missing_in_other_hello();
    }
}

class Base
{
}

class Child extends Base
{
    use Greets, Other {
        Greets::hello insteadof Other;
        hidden as protected;
        Greets::renamed as aliased;
        Other::other as otherAlias;
    }

    public function shadowed(): void
    {
    }
}

class Lone
{
    use Other;
}
