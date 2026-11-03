<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Code\IssueTrigger;

use function sprintf;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class IssueTrigger
{
    private ?Code $callee;
    private ?Code $caller;

    public static function from(?Code $callee, ?Code $caller): self
    {}

    private function __construct(?Code $callee, ?Code $caller)
    {}

    /**
     * An issue is triggered in first-party code or in test code.
     */
    public function isSelf(): bool
    {}

    /**
     * First-party code triggers an issue in third-party code.
     */
    public function isDirect(): bool
    {}

    /**
     * Third-party code triggers an issue.
     */
    public function isIndirect(): bool
    {}

    public function isUnknown(): bool
    {}

    /**
     * @return non-empty-string
     */
    public function callerAsString(): string
    {}

    /**
     * @return non-empty-string
     */
    public function calleeAsString(): string
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}

    /**
     * @return non-empty-string
     */
    private function codeAsString(?Code $code): string
    {}
}
