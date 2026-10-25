<?php

namespace TraitMemberControls;

trait Source
{
    public static function take(int $value): void
    {
    }
}

trait Left
{
    use Source;
}

trait Right
{
    use Source;
}

class Ordinary
{
    use Source;
}

class Diamond
{
    use Left, Right;
}

class Aliased
{
    use Source { take as renamed; }
}

trait Other
{
    public static function take(int $first, int $second): void
    {
    }
}

class Preferred
{
    use Source, Other {
        Source::take insteadof Other;
    }
}

class OwnMethod
{
    use Source;

    public static function take(): void
    {
    }
}

class ParentMethods
{
    public static function inherited(int $value): void
    {
    }
}

class Child extends ParentMethods
{
}

/** @method static void annotated() */
trait Tagged
{
}

trait TaggedLeft
{
    use Tagged;
}

trait TaggedRight
{
    use Tagged;
}

class AnnotatedDiamond
{
    use TaggedLeft, TaggedRight;
}

Ordinary::take();
Diamond::take();
Preferred::take();
Child::inherited();
AnnotatedDiamond::missing();

Ordinary::take(1);
Diamond::take(1);
Aliased::renamed(1);
Preferred::take(1);
OwnMethod::take();
Child::inherited(1);
AnnotatedDiamond::annotated();
