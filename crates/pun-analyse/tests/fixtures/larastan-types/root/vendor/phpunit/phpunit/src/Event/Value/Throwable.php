<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Code;

use const PHP_EOL;
use PHPUnit\Event\NoPreviousThrowableException;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Throwable
{
    /**
     * @var class-string
     */
    private string $className;
    private string $message;
    private string $description;
    private string $stackTrace;
    private ?Throwable $previous;

    /**
     * @param class-string $className
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(string $className, string $message, string $description, string $stackTrace, ?self $previous)
    {}

    /**
     * @throws NoPreviousThrowableException
     */
    public function asString(): string
    {}

    /**
     * @return class-string
     */
    public function className(): string
    {}

    public function message(): string
    {}

    public function description(): string
    {}

    public function stackTrace(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->previous
     */
    public function hasPrevious(): bool
    {}

    /**
     * @throws NoPreviousThrowableException
     */
    public function previous(): self
    {}
}
