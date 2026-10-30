<?php
namespace InitializerMetadataRoots;

/** @return Values::SPLIT_* */
function documentedWildcard() { return 1; }
\PHPStan\dumpType(documentedWildcard());

/** @param class-string<Values> $class */
function lateFinal(string $class): void {
    \PHPStan\dumpType($class::FINAL_SUM);
    \PHPStan\dumpType($class::FINAL_SPLIT);
}
