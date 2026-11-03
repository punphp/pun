<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class RequiresMethod extends Metadata
{
    /**
     * @var non-empty-string
     */
    private string $className;

    /**
     * @var non-empty-string
     */
    private string $methodName;

    /**
     * @param non-empty-string $className
     * @param non-empty-string $methodName
     */
    protected function __construct(Level $level, string $className, string $methodName)
    {}

    public function isRequiresMethod(): true
    {}

    /**
     * @return non-empty-string
     */
    public function className(): string
    {}

    /**
     * @return non-empty-string
     */
    public function methodName(): string
    {}
}
