<?php

namespace AttributeVersions;

#[\Attribute(\Attribute::TARGET_CLASS)]
class OnClass {}

#[OnClass]
const ONE = 1;

#[\Deprecated]
trait Deprecated {}

trait Plain {}

class Uses
{
    use Deprecated;
    use Plain;

    #[\Override]
    public int $a = 1;

    public function __construct(
        #[\Override]
        public int $b,
        #[OnClass]
        public int $c,
    ) {}
}

