<?php

namespace ThrowsControls;

use Exception as Problem;

/**
 * @throws bool
 * @phpstan-throws Problem
 */
function preferredValid(): void {}

/**
 * @throws Problem
 * @phpstan-throws string
 */
function preferredInvalid(): void {}

/**
 * @phpstan-throws (
 * @throws bool
 */
function invalidPreferredFallsBack(): void {}

/** @psalm-throws bool */
function unrelatedTag(): void {}

/**
 * @throws Problem
 * @throws string
 */
function multipleTags(): void {}

/** @throws never */
function impossible(): void {}

/** @throws mixed */
function unknown(): void {}

/** @throws null */
function nullType(): void {}

/** @throws class-string<Problem> */
function classNameNotObject(): void {}

/** @throws (int is int ? Problem : \stdClass) */
function bothBranchesMatter(): void {}

/** @throws (int is not string ? void : Problem) */
function negatedValid(): void {}

/** @throws (int is string ? Problem|void : Problem) */
function unionInsideBranch(): void {}

/**
 * @template T of Problem
 * @param T $exception
 * @throws T
 */
function throwableTemplate($exception): void
{
    /** @throws T */
    $copy = $exception;
}

/**
 * @template T
 * @param T $value
 * @throws T
 */
function unconstrainedTemplate($value): void {}

/** @phpstan-require-extends Problem */
interface Required {}

/** @psalm-require-extends Problem */
interface PsalmRequired {}

interface Plain {}

/** @phpstan-require-extends Problem */
class NotAnInterface {}

/** @phpstan-require-extends Problem */
trait NotAnInterfaceEither {}

/** @throws Required|PsalmRequired */
function requiredInterfaces(): void {}

/** @throws Plain */
function unconstrainedInterface(): void {}

/** @throws NotAnInterface */
function classConstraintDoesNotCount(): void {}

/** @throws NotAnInterfaceEither */
function traitConstraintDoesNotCount(): void {}

/** @throws Missing */
function unknownClass(): void {}

class Derived extends Problem
{
    /** @throws self|static|parent */
    public function validRelativeTypes(): void {}
}

/**
 * @template T of Required
 * @param T $value
 * @throws T
 */
function requiredTemplate($value): void {}

/**
 * @template T of Problem
 */
class Generic
{
    /** @throws T */
    public function inheritedClassTemplate(): void {}

    /**
     * @template U of Problem
     * @param U $value
     * @throws U
     */
    public function methodTemplate($value): void
    {
        /** @throws U */
        $copy = $value;
    }
}
