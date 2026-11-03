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

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestDox
{
    private string $prettifiedClassName;
    private string $prettifiedMethodName;
    private string $prettifiedAndColorizedMethodName;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(string $prettifiedClassName, string $prettifiedMethodName, string $prettifiedAndColorizedMethodName)
    {}

    public function prettifiedClassName(): string
    {}

    public function prettifiedMethodName(bool $colorize = false): string
    {}
}
