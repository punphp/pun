<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Arg;
use PhpParser\Node\ArgPlaceholder;
use PhpParser\Node\Expr;
use PhpParser\Node\VariadicPlaceholder;

abstract class CallLike extends Expr {
    /**
     * Return raw arguments, which may be actual Args, VariadicPlaceholders for first-class
     * callables, or ArgPlaceholders for partial function application.
     *
     * @return array<Arg|VariadicPlaceholder|ArgPlaceholder>
     */
    abstract public function getRawArgs(): array;

    /**
     * Returns whether this call expression is actually a first class callable.
     */
    public function isFirstClassCallable(): bool {}

    /**
     * Returns whether this call expression is a partial function application, i.e. whether its
     * argument list contains one or more "?" placeholders or a "..." placeholder. First-class
     * callables are a special case of partial function application, so this also returns true
     * for them.
     */
    public function isPartialFunctionApplication(): bool {}

    /**
     * Assert that this is not a partial function application (which includes first-class
     * callables) and return only ordinary Args.
     *
     * @return Arg[]
     */
    public function getArgs(): array {}

    /**
     * Retrieves a specific argument from the raw arguments.
     *
     * Returns the named argument that matches the given `$name`, or the
     * positional (unnamed) argument that exists at the given `$position`.
     * Returns `null` if no match is found, or when a "..." placeholder is
     * encountered, as argument positions are no longer known past that point.
     * A "?" placeholder occupies its argument position, but is never returned,
     * as it is not an actual argument.
     */
    public function getArg(string $name, int $position): ?Arg {}
}
