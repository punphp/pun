<?php
namespace ClassCaseOptions\Implicit {
    use ClassCaseOptions\Definitions\Marker;
    use ClassCaseOptions\Definitions\Failure;
    use ClassCaseOptions\Definitions\Contract;
    use ClassCaseOptions\Definitions\Feature;
    use ClassCaseOptions\Definitions\Kind;

    #[marker]
    class Attributed {}

    class Hierarchy extends marker implements contract
    {
        use feature;
        public marker $property;

        public function __construct(public ?marker $value = null) {}
    }

    function signature(marker $value, marker|\stdClass $other): marker
    {
        $value instanceof marker;
        try {} catch (failure $e) {}
        return $value;
    }

    new marker();
    marker::make();
    marker::VALUE;
    marker::$value;
    $callable = marker::make(...);
    marker::class;
    kind::One;
    $closure = function (marker $value): marker { return $value; };
    $arrow = fn (marker $value): marker => $value;
}

namespace ClassCaseOptions\Explicit {
    use ClassCaseOptions\Definitions\Marker as Alias;
    use ClassCaseOptions\Definitions\MARKER as Shouted;

    #[Alias]
    class ExactAlias {}
    #[alias]
    class WrongAliasCase {}
    #[Shouted]
    class ExactShouted {}
    #[shouted]
    class WrongShoutedCase {}

    new Alias();
    new alias();
    new Shouted();
    new shouted();
    Alias::make();
    alias::VALUE;
    $callback = alias::make(...);
    function unionNode(Alias|\stdClass $value): Alias|\stdClass { return $value; }
}

namespace ClassCaseOptions\Qualified {
    use ClassCaseOptions\Definitions as Prefix;

    #[Prefix\marker]
    class LowerFinalComponent {}
    #[prefix\Marker]
    class LowerPrefix {}
    #[\ClassCaseOptions\Definitions\marker]
    class FullyQualified {}

    new Prefix\marker();
    new prefix\Marker();
    new \ClassCaseOptions\Definitions\marker();
}

namespace ClassCaseOptions\Grouped {
    use ClassCaseOptions\Definitions\{Marker, MARKER as GroupAlias};

    #[marker]
    class Implicit {}
    #[GroupAlias]
    class Explicit {}
    #[groupalias]
    class WrongAliasCase {}
}

namespace ClassCaseOptions\Relative {
    #[\Attribute]
    class Marker {}

    #[namespace\marker]
    class Relative {}
    new namespace\marker();
}

namespace ClassCaseOptions\Reset {
    #[\Attribute]
    class Alias {}

    #[alias]
    class NamespaceClearsExplicitAliases {}
}
