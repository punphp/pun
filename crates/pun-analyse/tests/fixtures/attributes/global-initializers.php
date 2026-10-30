<?php

namespace {
    const ATTRIBUTE_GLOBAL_ZERO = 0;
    const ATTRIBUTE_SHADOWED_ZERO = 0;
}

namespace GlobalInitializerReflection {
    use const GlobalInitializerScanned\ZERO as IMPORTED_ZERO;
    use const GlobalInitializerScanned\NIL as IMPORTED_NIL;
    use const GlobalInitializerScanned\SPLIT as IMPORTED_SPLIT;

    function beforeDeclarations(): void
    {
        \PHPStan\dumpType(SPLIT);
        \PHPStan\dumpType(ZERO_COALESCE);
        \PHPStan\dumpType(FALSEY_MIDDLE);
        \PHPStan\dumpType(SHAPE);
        $keys = [SPLIT => 'union', 1 => 'not necessarily repeated'];
    }

    const ZERO = 0;
    const NIL = null;
    const FALSE_VALUE = false;
    const EMPTY_ARRAY = [];
    const FALSEY_STRING = '0';
    const ATTRIBUTE_SHADOWED_ZERO = 4;
    const SPLIT = true ? 1 : 4;
    const ZERO_COALESCE = 0 ?? 1;
    const FALSEY_MIDDLE = true ? 0 : 1;
    const FORWARD = LATE_ZERO ?: 1;
    const LATE_ZERO = 0;
    const REMAINDER = 5.9 % 2.0;
    const RICH_MIDDLE = false ? (5.9 % 2.0) : 1;
    const SHAPE = [SPLIT, 'coalesce' => ZERO_COALESCE, 'precise' => RICH_MIDDLE];
    const NULL_CHAIN = IMPORTED_NIL ?? 1;
    const GLOBAL_FALLBACK = ATTRIBUTE_GLOBAL_ZERO ?: 1;
    const NARROWED_CONDITION = SPLIT === 1 ? SPLIT : 1;

    function afterDeclarations(): void
    {
        \PHPStan\dumpType(SPLIT);
        \PHPStan\dumpType(ZERO_COALESCE);
        \PHPStan\dumpType(FALSEY_MIDDLE);
        \PHPStan\dumpType(SHAPE);
        \PHPStan\dumpType(IMPORTED_SPLIT);
        \PHPStan\dumpType(FORWARD);
        \PHPStan\dumpType(RICH_MIDDLE);
        \PHPStan\dumpType(NULL_CHAIN);
        \PHPStan\dumpType(GLOBAL_FALLBACK);
        $copy = SPLIT;
        \PHPStan\dumpType($copy);
        $copy = 9;
        \PHPStan\dumpType($copy);
        $keys = [SPLIT => 'runtime', 1 => 'repeated'];
        \PHPStan\dumpType(true ? 1 : 4);
        \PHPStan\dumpType(0 ?? 1);
        \PHPStan\dumpType(true ? 0 : 1);
    }

    #[\Attribute(ZERO ?: 1)] class ShortZero {}
    #[\Attribute(true ? ZERO : 1)] class FullZero {}
    #[\Attribute(NIL ?? 1)] class NullLeft {}
    #[\Attribute(FALSE_VALUE ?: 1)] class FalseLeft {}
    #[\Attribute(EMPTY_ARRAY ?: 1)] class EmptyLeft {}
    #[\Attribute(FALSEY_STRING ?: 1)] class StringLeft {}
    #[\Attribute(false ? REMAINDER : 1)] class RichReference {}
    #[\Attribute(RICH_MIDDLE)] class RichInitializer {}
    #[\Attribute(FORWARD)] class ForwardReference {}
    #[\Attribute(NULL_CHAIN)] class ScannedNullReference {}
    #[\Attribute(GLOBAL_FALLBACK)] class GlobalFallbackInitializer {}
    #[\Attribute(ATTRIBUTE_GLOBAL_ZERO ?: 1)] class GlobalFallbackArgument {}
    #[\Attribute(IMPORTED_ZERO ?: 1)] class ImportedAlias {}
    #[\Attribute(\GlobalInitializerScanned\ZERO ?: 1)] class QualifiedReference {}
    #[\Attribute(FALSEY_MIDDLE)] class FalseyReflectionNotFlow {}
    #[\Attribute(((ZERO ?: 1) | 128))] class NestedRepeatable {}
    #[\Attribute(SPLIT)] class SplitReflectionNotFlow {}
    #[\Attribute(ZERO_COALESCE)] class ZeroReflectionNotFlow {}
    #[\Attribute(ATTRIBUTE_SHADOWED_ZERO ?: 1)] class NamespaceStopsFallback {}
    #[\Attribute(IMPORTED_SPLIT ?: 1)] class ScannedDistinct {}
    #[\Attribute(ZERO ?? 1)] class ZeroNotNull {}
    #[\Attribute(NIL ?: 0)] class ZeroFlags {}
    #[\Attribute(NIL ?? 4)] class MethodOnly {}
    #[\Attribute(SHAPE)] class NonInteger {}
    #[\Attribute(UNKNOWN ?: 1)] class Unresolved {}
    #[\Attribute(NARROWED_CONDITION)] class DoNotNarrowReflection {}

    #[ShortZero, FullZero, NullLeft, FalseLeft, EmptyLeft, StringLeft]
    #[RichReference, RichInitializer, ForwardReference, ScannedNullReference]
    #[GlobalFallbackInitializer, GlobalFallbackArgument, ImportedAlias, QualifiedReference]
    #[FalseyReflectionNotFlow, NestedRepeatable, NestedRepeatable]
    #[SplitReflectionNotFlow, ZeroReflectionNotFlow, NamespaceStopsFallback, ScannedDistinct]
    #[ZeroNotNull, ZeroFlags, MethodOnly, NonInteger]
    #[Unresolved, DoNotNarrowReflection]
    #[\GlobalInitializerScannedAttributes\ScannedValid]
    #[\GlobalInitializerScannedAttributes\ScannedInvalid]
    class Consumer
    {
        #[MethodOnly] public function supported(): void {}
    }
}
