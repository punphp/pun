<?php
function returned(array $values): void {
    return;
    $values[];
    missingAfterReturn();
}
function thrown(array $values): void {
    throw new \RuntimeException();
    $values[];
}
function exited(array $values): void {
    exit(0);
    $values[];
}
function bothBranches(array $values, bool $condition): void {
    if ($condition) {
        return;
    } else {
        return;
    }
    $values[];
}
function oneBranch(array $values, bool $condition): void {
    echo 'reachable';
    if ($condition) {
        return;
    }
    $copy = $values[];
}
function loopBody(array $values): void {
    foreach ($values as $value) {
        echo 'reachable';
        continue;
        $values[];
    }
}
function declarations(): void {
    return;
    function declaredAfterReturn(): void {
        missingFromDeclaration();
    }
    class DeclaredAfterReturn {
        public function run(): void {
            missingFromMethod();
        }
    }
}
