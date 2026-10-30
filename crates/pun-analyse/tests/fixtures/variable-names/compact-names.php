<?php

namespace CompactNameAliases {
    use function compact as collect;
    use function \compact as collectAbsolute;
    use function CoMpAcT as MixedAlias;

    /** @param 'typedMissing' $name */
    function aliases(string $name, bool $flag): void {
        collect('aliasMissing');
        collectAbsolute('absoluteMissing');
        mIXEDaLIAS('caseMissing');
        \CoMpAcT('fullyMissing');
        collect($name);
        collect(['arrayMissing', ['nestedMissing']]);
        collect($assigned = 'assigned');
        echo $assigned;
        collect('later', $later = 'later');
        echo $later;
        if ($flag) {
            $maybe = 1;
        }
        collect('maybe');
        $defined = 1;
        collect('defined');
        $dynamic = 'compact';
        $dynamic('dynamicIgnored');
        ('compact')('parenthesizedStringIgnored');
        $callback = collect(...);
        $callback('firstClassIgnored');
    }
}

namespace CompactNameOther {
    use function CompactNameProvider\user as compact;

    function importedUser(): void {
        cOmPaCt('userIgnored');
        \compact('absoluteSelected');
        compact($assigned = 'assigned');
        echo $assigned;
    }
}

namespace CompactNameGrouped {
    use function CompactNameProvider\{user as compact};

    function groupedUser(): void {
        compact('groupedIgnored');
    }
}

namespace CompactNameMixedGroup {
    use CompactNameProvider\{function user as compact};

    function mixedGroupedUser(): void {
        compact('mixedGroupedIgnored');
    }
}

namespace CompactNameLocal {
    function compact(string $name): array { return []; }

    function local(): void {
        compact('localMissing');
        COMPACT('localCaseMissing');
        namespace\compact('relativeIgnored');
        \CompactNameLocal\compact('qualifiedIgnored');
        \compact('globalMissing');
        $dynamic = 'CompactNameLocal\\compact';
        $dynamic('localDynamicIgnored');
    }
}

namespace CompactNameQualified {
    use CompactNameProvider as Provider;
    use CompactNameProvider as compact;

    function qualified(): void {
        Provider\compact('namespaceAliasIgnored');
        compact\compact('compactNamespaceIgnored');
        CompactNameProvider\compact('qualifiedIgnored');
        namespace\compact('relativeIgnored');
        compact('defaultImportMissing');
    }
}

namespace CompactNameImported {
    use function compact;

    function imported(): void {
        compact('importedMissing');
        COMPACT('importedCaseMissing');
    }
}

namespace CompactNameAliases {
    function resetImports(): void {
        collect('resetIgnored');
        compact('resetMissing');
    }
}

namespace CompactNameProvider {
    function user(string $name): array { return []; }
    function compact(string $name): array { return []; }
}

namespace {
    function globals(bool $flag): void {
        compact('globalUnqualifiedMissing');
        \compact('globalAbsoluteMissing');
        namespace\compact('globalRelativeMissing');
        CoMpAcT('globalCaseMissing');
        compact($assigned = 'assigned');
        echo $assigned;
        $dynamic = 'compact';
        $dynamic('globalDynamicIgnored');
        if ($flag) {
            $maybe = 1;
        }
        compact('maybe');
    }
}
