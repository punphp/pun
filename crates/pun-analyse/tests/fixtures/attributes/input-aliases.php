<?php

namespace AttributeInputAliases\Definitions {
    #[\Attribute]
    class Marker
    {
        public function __construct(int $value = 0) {}
    }
}

namespace AttributeInputAliases\Explicit {
    use AttributeInputAliases\Definitions\MARKER as Alias;

    #[Alias]
    class Exact {}

    #[Alias(1)]
    class WithArguments {}

    #[alias]
    class WrongAliasCase {}
}

namespace AttributeInputAliases\Implicit {
    use AttributeInputAliases\Definitions\MARKER;

    #[MARKER]
    class Implicit {}
}

namespace AttributeInputAliases\Qualified {
    use AttributeInputAliases\Definitions as Prefix;

    #[Prefix\MARKER]
    class Qualified {}

    #[\AttributeInputAliases\Definitions\MARKER]
    class FullyQualified {}
}

namespace AttributeInputAliases\Grouped {
    use AttributeInputAliases\Definitions\{MARKER as GroupAlias};

    #[GroupAlias(1)]
    class Exact {}

    #[groupalias]
    class WrongAliasCase {}
}
