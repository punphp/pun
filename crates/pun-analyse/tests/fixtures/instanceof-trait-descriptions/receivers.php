<?php

namespace InitializationCoverage {
    trait MarkerTrait {}

    function traitReceiver(object $receiver): bool
    {
        return $receiver instanceof MarkerTrait;
    }
}

namespace InstanceofTraitDescriptions\Declarations {
    trait Marker {}

    class Receiver {}
    class OtherReceiver {}

    /** @template T */
    class Box {}

    trait ChecksReceiver
    {
        public function traitThis(): bool
        {
            return $this instanceof Marker;
        }

        public function traitParameter(object $receiver): bool
        {
            return $receiver instanceof Marker;
        }
    }

    class ParentReceiver
    {
        use ChecksReceiver;

        public function classThis(): bool
        {
            return $this instanceof Marker;
        }

        public function copiedThis(): bool
        {
            $receiver = $this;
            return $receiver instanceof Marker;
        }
    }

    class ChildReceiver extends ParentReceiver {}

    class OtherHost
    {
        use ChecksReceiver;
    }
}

namespace InstanceofTraitDescriptions\Consumers {
    use InstanceofTraitDescriptions\Declarations\Box;
    use InstanceofTraitDescriptions\Declarations\ChildReceiver;
    use InstanceofTraitDescriptions\Declarations\Marker as ImportedMarker;
    use InstanceofTraitDescriptions\Declarations\OtherReceiver;
    use InstanceofTraitDescriptions\Declarations\ParentReceiver;
    use InstanceofTraitDescriptions\Declarations\Receiver as ImportedReceiver;

    function explicitMixed(mixed $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function implicitMixed($receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function named(ImportedReceiver $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function nullable(?ImportedReceiver $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function alternatives(ImportedReceiver|OtherReceiver $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    /** @param Box<int> $receiver */
    function generic(Box $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    /** @param Box<int>|ImportedReceiver $receiver */
    function genericAlternatives(object $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function inherited(ChildReceiver $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function integer(int $receiver): bool
    {
        return $receiver instanceof ImportedMarker;
    }

    function literal(): bool
    {
        return 'a literal value' instanceof ImportedMarker;
    }

    function construction(): bool
    {
        return new ImportedReceiver() instanceof ImportedMarker;
    }

    function fullyQualified(object $receiver): bool
    {
        return $receiver instanceof \InstanceofTraitDescriptions\Declarations\Marker;
    }

    function caseControl(object $receiver): bool
    {
        return $receiver instanceof \InstanceofTraitDescriptions\Declarations\MARKER;
    }

    function ordinaryClass(ChildReceiver $receiver): bool
    {
        return $receiver instanceof ParentReceiver;
    }

    function dynamicClass(object $receiver, string $class): bool
    {
        return $receiver instanceof $class;
    }

    function invalidClassExpression(object $receiver, int $class): bool
    {
        return $receiver instanceof $class;
    }

    function missingClass(object $receiver): bool
    {
        return $receiver instanceof MissingReceiver;
    }
}
