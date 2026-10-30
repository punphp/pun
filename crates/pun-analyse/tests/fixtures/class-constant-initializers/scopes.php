<?php
namespace InitializerScopes {
    interface ParentFlags {
        const ZERO = 0;
        const SPLIT = true ? 1 : 4;
        const FILTER = self::ZERO ?: 1;
        const SHAPE = [self::SPLIT, self::FILTER];
    }
    interface ChildFlags extends ParentFlags {}
    class Implementer implements ChildFlags {}
    \PHPStan\dumpType(ChildFlags::SPLIT);
    \PHPStan\dumpType(Implementer::FILTER);
    \PHPStan\dumpType(Implementer::SHAPE);

    class Grand { const FLAG = 0; }
    class Base extends Grand {
        const FLAG = 4;
        const SELF_COPY = self::FLAG;
        const PARENT_COPY = parent::FLAG;
        const SELF_FILTER = self::FLAG ?: 1;
        const PARENT_FILTER = parent::FLAG ?: 1;
        const PRECISE = (self::FLAG + 0.5) * 2;
        const NESTED = [(self::FLAG ?: 1) | 16, (parent::FLAG ?? 1)];
    }
    /** @return MetadataOnly::SPLIT */
    function documentedOnly() { return 1; }
    \PHPStan\dumpType(documentedOnly());
}
namespace InitializerOther {
    use InitializerScopes\Base as Alias;
    class Child extends Alias { const FLAG = 16; }
    \PHPStan\dumpType(Child::SELF_COPY);
    \PHPStan\dumpType(Child::PARENT_COPY);
    \PHPStan\dumpType(Child::SELF_FILTER);
    \PHPStan\dumpType(Child::PARENT_FILTER);
    \PHPStan\dumpType(Child::PRECISE);
    \PHPStan\dumpType(Child::NESTED);
    #[\Attribute(self::FLAG ?: 1)] class InvalidSelf { const FLAG = 4; }
    #[\Attribute(parent::PARENT_FILTER)] class ValidParent extends Child {}
    #[\Attribute(\InitializerScopes\Implementer::FILTER)] class ValidInterface {}
    #[InvalidSelf, ValidParent, ValidInterface] class Consumer {}
}
