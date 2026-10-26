<?php

namespace {
    const SCANNER_ZERO = 0;
    const SCANNER_SHADOWED = 0;
    const SCANNER_MISSING_ALIAS = 0;
}

namespace GlobalInitializerDependencies {
    use const SCANNER_ZERO as IMPORTED;
    use const Missing\NO_SUCH_CONSTANT as SCANNER_MISSING_ALIAS;

    const SCANNER_SHADOWED = 4;
    const NIL = null;
    const ZERO = 0;

    #[\Attribute(IMPORTED ?: 1)] class ExplicitAlias {}
    #[\Attribute(SCANNER_ZERO ?: 1)] class GlobalFallback {}
    #[\Attribute(\SCANNER_ZERO ?: 1)] class QualifiedGlobal {}
    #[\Attribute(NIL ?? 1)] class RemoveNull {}
    #[\Attribute(true ? ZERO : 1)] class FullTernary {}
    #[\Attribute(SCANNER_SHADOWED ?: 1)] class ExistingStopsFallback {}
    #[\Attribute(namespace\SCANNER_ZERO ?: 1)] class QualifiedDoesNotFallBack {}
    #[\Attribute(SCANNER_MISSING_ALIAS ?: 1)] class AliasDoesNotFallBack {}
    #[\Attribute(ZERO ?? 1)] class ZeroIsNotNull {}
    #[\Attribute(\max(1, ZERO))] class UnsupportedCall {}
    #[\Attribute(true ? new \stdClass : ZERO)] class UnsupportedConstruction {}

    #[ExplicitAlias, GlobalFallback, QualifiedGlobal, RemoveNull, FullTernary]
    #[ExistingStopsFallback, QualifiedDoesNotFallBack, AliasDoesNotFallBack, ZeroIsNotNull]
    #[UnsupportedCall, UnsupportedConstruction]
    class Consumer {}
}
