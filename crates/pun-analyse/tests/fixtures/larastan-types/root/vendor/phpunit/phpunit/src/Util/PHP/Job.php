<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util\PHP;

use PHPUnit\Event\TestRunner\ChildProcessReason;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Job
{
    /**
     * @var non-empty-string
     */
    private string $code;
    private ChildProcessReason $reason;

    /**
     * @var list<string>
     */
    private array $phpSettings;

    /**
     * @var array<string, string>
     */
    private array $environmentVariables;

    /**
     * @var list<non-empty-string>
     */
    private array $arguments;

    /**
     * @var ?non-empty-string
     */
    private ?string $input;
    private bool $redirectErrors;
    private bool $requiresXdebug;

    /**
     * @param non-empty-string       $code
     * @param list<string>           $phpSettings
     * @param array<string, string>  $environmentVariables
     * @param list<non-empty-string> $arguments
     * @param ?non-empty-string      $input
     */
    public function __construct(string $code, ChildProcessReason $reason, array $phpSettings = [], array $environmentVariables = [], array $arguments = [], ?string $input = null, bool $redirectErrors = false, bool $requiresXdebug = false)
    {}

    /**
     * @return non-empty-string
     */
    public function code(): string
    {}

    public function reason(): ChildProcessReason
    {}

    /**
     * @return list<string>
     */
    public function phpSettings(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->environmentVariables
     */
    public function hasEnvironmentVariables(): bool
    {}

    /**
     * @return array<string, string>
     */
    public function environmentVariables(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->arguments
     */
    public function hasArguments(): bool
    {}

    /**
     * @return list<non-empty-string>
     */
    public function arguments(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->input
     */
    public function hasInput(): bool
    {}

    /**
     * @throws PhpProcessException
     *
     * @return non-empty-string
     */
    public function input(): string
    {}

    public function redirectErrors(): bool
    {}

    public function requiresXdebug(): bool
    {}
}
