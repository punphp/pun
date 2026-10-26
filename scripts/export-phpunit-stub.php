<?php

// Export the declarations of the PHPUnit classes rector-phpunit's rules ask PHPStan about, for pun-lint's fixture
// tests (rector-phpunit runs its fixtures with phpunit/phpunit ^13.2 installed).
//
//   php scripts/export-phpunit-stub.php <project with phpunit 13.2 in vendor/> crates/pun-lint/tests/phpunit-stubs
//
// Each class's file is copied to <out>/<namespace path>/<class>.php as written, with the bodies of its methods left
// empty, so the imports, doc comments and signatures stay as PHPUnit has them.

[$_, $project, $outDir] = $argv + [null, null, null];
require $project.'/vendor/autoload.php';

$classes = [
    'PHPUnit\Framework\Assert',
    'PHPUnit\Framework\TestCase',
    'PHPUnit\Framework\Test',
    'PHPUnit\Framework\SelfDescribing',
    'PHPUnit\Framework\Reorderable',
    'PHPUnit\Framework\MockObject\MockObject',
    'PHPUnit\Framework\MockObject\Stub',
    'PHPUnit\Framework\MockObject\InvocationMocker',
    'PHPUnit\Framework\MockObject\InvocationStubber',
    'PHPUnit\Framework\MockObject\MockBuilder',
    'PHPUnit\Framework\MockObject\Rule\InvocationOrder',
    'PHPUnit\Framework\MockObject\Rule\AnyInvokedCount',
    'PHPUnit\Framework\MockObject\Rule\InvokedCount',
    'PHPUnit\Framework\MockObject\Rule\InvokedAtLeastOnce',
    'PHPUnit\Framework\MockObject\Rule\InvokedAtLeastCount',
    'PHPUnit\Framework\MockObject\Rule\InvokedAtMostCount',
    'PHPUnit\Framework\MockObject\Stub\Stub',
    'PHPUnit\Framework\Constraint\Constraint',
    'PHPUnit\Framework\Constraint\IsEqual',
    'PHPUnit\Framework\Constraint\IsIdentical',
    'PHPUnit\Framework\Constraint\IsInstanceOf',
    'PHPUnit\Framework\Constraint\Callback',
    'PHPUnit\Framework\Constraint\IsType',
    'PHPUnit\Framework\Constraint\IsAnything',
    // the attributes rector-phpunit's annotation rules write (`reflectionProvider->hasClass()`)
    'PHPUnit\Framework\Attributes\AllowMockObjectsWithoutExpectations',
    'PHPUnit\Framework\Attributes\BackupGlobals',
    'PHPUnit\Framework\Attributes\CoversClass',
    'PHPUnit\Framework\Attributes\CoversFunction',
    'PHPUnit\Framework\Attributes\CoversMethod',
    'PHPUnit\Framework\Attributes\CoversTrait',
    'PHPUnit\Framework\Attributes\DataProvider',
    'PHPUnit\Framework\Attributes\DataProviderExternal',
    'PHPUnit\Framework\Attributes\Depends',
    'PHPUnit\Framework\Attributes\DependsExternal',
    'PHPUnit\Framework\Attributes\DependsOnClass',
    'PHPUnit\Framework\Attributes\DependsUsingDeepClone',
    'PHPUnit\Framework\Attributes\DoesNotPerformAssertions',
    'PHPUnit\Framework\Attributes\RequiresFunction',
    'PHPUnit\Framework\Attributes\RequiresMethod',
    'PHPUnit\Framework\Attributes\RequiresOperatingSystem',
    'PHPUnit\Framework\Attributes\RequiresOperatingSystemFamily',
    'PHPUnit\Framework\Attributes\RequiresPhp',
    'PHPUnit\Framework\Attributes\RequiresPhpExtension',
    'PHPUnit\Framework\Attributes\RequiresPhpunit',
    'PHPUnit\Framework\Attributes\RequiresSetting',
    'PHPUnit\Framework\Attributes\Test',
    'PHPUnit\Framework\Attributes\TestWith',
    'PHPUnit\Framework\Attributes\Ticket',
];

/** The code of a file with the body of each method of its class-likes left empty. */
function without_method_bodies(string $code): string
{
    $out = '';
    $depth = 0;
    $classDepth = null;
    $inMethod = false;
    $skipFrom = null;
    $pendingClass = false;
    foreach (token_get_all($code) as $token) {
        $text = is_array($token) ? $token[1] : $token;
        $id = is_array($token) ? $token[0] : null;
        if ($skipFrom !== null) {
            if ($text === '{' || $id === T_CURLY_OPEN || $id === T_DOLLAR_OPEN_CURLY_BRACES) {
                $depth++;
            } elseif ($text === '}') {
                $depth--;
                if ($depth === $skipFrom) {
                    $skipFrom = null;
                    $out .= "\n    }";
                }
            }
            continue;
        }
        if (in_array($id, [T_CLASS, T_INTERFACE, T_TRAIT, T_ENUM], true)) {
            $pendingClass = true;
        }
        if ($id === T_FUNCTION && $classDepth !== null && $depth === $classDepth + 1) {
            $inMethod = true;
        }
        if ($text === ';' && $inMethod) {
            $inMethod = false;
        }
        if ($text === '{' || $id === T_CURLY_OPEN || $id === T_DOLLAR_OPEN_CURLY_BRACES) {
            if ($pendingClass) {
                $pendingClass = false;
                $classDepth = $depth;
            } elseif ($inMethod) {
                $inMethod = false;
                $out .= "{";
                $skipFrom = $depth;
                $depth++;
                continue;
            }
            $depth++;
        } elseif ($text === '}') {
            $depth--;
            if ($classDepth !== null && $depth === $classDepth) {
                $classDepth = null;
            }
        }
        $out .= $text;
    }

    return $out;
}

foreach ($classes as $name) {
    $file = (new ReflectionClass($name))->getFileName();
    $target = $outDir.'/'.str_replace('\\', '/', preg_replace('/^PHPUnit\\\\/', '', $name)).'.php';
    @mkdir(dirname($target), 0777, true);
    file_put_contents($target, without_method_bodies(file_get_contents($file)));
}
